use crate::error::{Error, Result};
use reqwest::multipart::Form;
use reqwest::{Client, Proxy, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::time::Duration;
use tracing::{debug, warn};

const DEFAULT_MAX_RETRIES: u32 = 5;
const INITIAL_BACKOFF: Duration = Duration::from_secs(2);
const MAX_BACKOFF: Duration = Duration::from_secs(60);

#[derive(Debug, Clone, Copy)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

#[derive(Debug, Clone)]
pub struct ApiClientBuilder {
    base_url: String,
    token: String,
    proxy: Option<String>,
    max_retries: u32,
    timeout: Duration,
}

impl ApiClientBuilder {
    pub fn new(base_url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            token: token.into(),
            proxy: None,
            max_retries: DEFAULT_MAX_RETRIES,
            timeout: Duration::from_secs(30),
        }
    }

    pub fn proxy(mut self, proxy_url: impl Into<String>) -> Self {
        self.proxy = Some(proxy_url.into());
        self
    }

    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn build(self) -> Result<ApiClient> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", self.token))
                .map_err(|err| Error::InvalidHeader(err.to_string()))?,
        );
        headers.insert(reqwest::header::ACCEPT, "application/json".parse().unwrap());

        let mut builder = Client::builder()
            .default_headers(headers)
            .timeout(self.timeout);
        if let Some(proxy_url) = &self.proxy {
            builder = builder.proxy(Proxy::all(proxy_url).map_err(Error::InvalidProxy)?);
        }

        Ok(ApiClient {
            http: builder.build().map_err(Error::Http)?,
            base_url: self.base_url,
            max_retries: self.max_retries,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ApiClient {
    pub(crate) http: Client,
    pub(crate) base_url: String,
    pub(crate) max_retries: u32,
}

impl ApiClient {
    pub fn builder(base_url: impl Into<String>, token: impl Into<String>) -> ApiClientBuilder {
        ApiClientBuilder::new(base_url, token)
    }

    pub async fn request_json<Q, B, R>(
        &self,
        method: HttpMethod,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<R>
    where
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        self.execute(method, path, query, move |request| {
            if let Some(body) = body {
                Ok(request.json(body))
            } else {
                Ok(request)
            }
        })
        .await
    }

    pub async fn request_multipart<Q, B, R>(
        &self,
        method: HttpMethod,
        path: &str,
        query: Option<&Q>,
        body: &B,
    ) -> Result<R>
    where
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let value = serde_json::to_value(body).map_err(Error::Json)?;
        self.execute(method, path, query, move |request| {
            let form = multipart_from_value(value.clone())?;
            Ok(request.multipart(form))
        })
        .await
    }

    async fn execute<Q, R, F>(
        &self,
        method: HttpMethod,
        path: &str,
        query: Option<&Q>,
        decorate: F,
    ) -> Result<R>
    where
        Q: Serialize + ?Sized,
        R: DeserializeOwned,
        F: Fn(reqwest::RequestBuilder) -> Result<reqwest::RequestBuilder>,
    {
        let url = if path.starts_with("http://") || path.starts_with("https://") {
            path.to_string()
        } else {
            format!(
                "{}/{}",
                self.base_url.trim_end_matches('/'),
                path.trim_start_matches('/')
            )
        };

        let mut backoff = INITIAL_BACKOFF;
        for attempt in 0..=self.max_retries {
            let mut request = match method {
                HttpMethod::Get => self.http.get(&url),
                HttpMethod::Post => self.http.post(&url),
                HttpMethod::Put => self.http.put(&url),
                HttpMethod::Delete => self.http.delete(&url),
                HttpMethod::Patch => self.http.patch(&url),
            };
            if let Some(query) = query {
                request = request.query(query);
            }
            request = decorate(request)?;

            debug!(attempt, url = %url, "sending request");
            let response = match request.send().await {
                Ok(response) => response,
                Err(err) if err.is_timeout() && attempt < self.max_retries => {
                    warn!(attempt, "request timed out, retrying in {:?}", backoff);
                    tokio::time::sleep(backoff).await;
                    backoff = (backoff * 2).min(MAX_BACKOFF);
                    continue;
                }
                Err(err) => return Err(Error::Http(err)),
            };

            let status = response.status();
            if matches!(
                status,
                StatusCode::TOO_MANY_REQUESTS
                    | StatusCode::BAD_GATEWAY
                    | StatusCode::SERVICE_UNAVAILABLE
            ) {
                let retry_after = response
                    .headers()
                    .get(reqwest::header::RETRY_AFTER)
                    .and_then(|header| header.to_str().ok())
                    .and_then(|value| value.parse::<u64>().ok());
                let response_text = response.text().await.map_err(Error::Http)?;

                if attempt < self.max_retries {
                    let wait_for = retry_after.map(Duration::from_secs).unwrap_or(backoff);
                    warn!(
                        attempt,
                        status = status.as_u16(),
                        "retrying in {:?}",
                        wait_for
                    );
                    tokio::time::sleep(wait_for).await;
                    backoff = (backoff * 2).min(MAX_BACKOFF);
                    continue;
                }

                return if status == StatusCode::TOO_MANY_REQUESTS {
                    Err(Error::RateLimited {
                        attempts: self.max_retries + 1,
                        retry_after,
                        body: response_text,
                    })
                } else {
                    Err(Error::RetryExhausted {
                        status: status.as_u16(),
                        attempts: self.max_retries + 1,
                        body: response_text,
                    })
                };
            }

            let status_code = status.as_u16();
            let response_text = response.text().await.map_err(Error::Http)?;
            if !status.is_success() {
                return Err(Error::Api {
                    status: status_code,
                    body: response_text,
                });
            }

            let payload = if response_text.trim().is_empty() {
                "null"
            } else {
                response_text.as_str()
            };
            return serde_json::from_str(payload).map_err(Error::Json);
        }

        Err(Error::RetryExhausted {
            status: 503,
            attempts: self.max_retries + 1,
            body: String::new(),
        })
    }
}

fn multipart_from_value(value: Value) -> Result<Form> {
    let Value::Object(fields) = value else {
        return Err(Error::InvalidMultipart(
            "multipart payload must serialize into an object".to_string(),
        ));
    };

    let mut form = Form::new();
    for (field, value) in fields {
        match value {
            Value::Null => {}
            Value::String(text) => {
                form = form.text(field, text);
            }
            Value::Number(number) => {
                form = form.text(field, number.to_string());
            }
            Value::Bool(flag) => {
                form = form.text(field, flag.to_string());
            }
            Value::Array(values) => {
                for item in values {
                    form = form.text(field.clone(), scalar_to_string(item)?);
                }
            }
            Value::Object(object) => {
                form = form.text(
                    field,
                    serde_json::to_string(&Value::Object(object)).map_err(Error::Json)?,
                );
            }
        }
    }

    Ok(form)
}

fn scalar_to_string(value: Value) -> Result<String> {
    match value {
        Value::Null => Ok(String::new()),
        Value::String(text) => Ok(text),
        Value::Number(number) => Ok(number.to_string()),
        Value::Bool(flag) => Ok(flag.to_string()),
        Value::Array(array) => serde_json::to_string(&Value::Array(array)).map_err(Error::Json),
        Value::Object(object) => serde_json::to_string(&Value::Object(object)).map_err(Error::Json),
    }
}
