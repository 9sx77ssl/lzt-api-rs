use lzt_api::{forum::ThreadsCreateRequest, Error, LolzteamClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const TOKEN: &str = "PASTE_YOUR_FULL_ACCESS_TOKEN_HERE";
    const FORUM_ID: i64 = 0;
    const PROXY: Option<&str> = None;

    let mut builder = LolzteamClient::builder(TOKEN).max_retries(5);

    if let Some(proxy) = PROXY {
        if !proxy.trim().is_empty() {
            builder = builder.proxy(proxy);
        }
    }

    let client = builder.build()?;

    let response = match client
        .forum()
        .threads_create(ThreadsCreateRequest {
            forum_id: FORUM_ID,
            title: Some("Всем привет".to_string()),
            post_body: "как ваши дела?".to_string(),
            watch_thread: Some(true),
            ..Default::default()
        })
        .await
    {
        Ok(response) => response,
        Err(Error::Api { status, body }) => {
            eprintln!("API returned HTTP {status}");
            eprintln!("{body}");
            return Ok(());
        }
        Err(Error::RateLimited {
            attempts,
            retry_after,
            body,
        }) => {
            eprintln!("API rate limited after {attempts} attempt(s)");
            if let Some(seconds) = retry_after {
                eprintln!("Retry-After: {seconds} second(s)");
            }
            eprintln!("{body}");
            return Ok(());
        }
        Err(Error::RetryExhausted {
            status,
            attempts,
            body,
        }) => {
            eprintln!("API returned HTTP {status} after {attempts} attempt(s)");
            if !body.is_empty() {
                eprintln!("{body}");
            }
            return Ok(());
        }
        Err(err) => return Err(err.into()),
    };

    println!("Thread created successfully");
    println!("Thread id: {}", response.thread.thread_id);
    println!("Title: {}", response.thread.thread_title);
    println!("URL: {}", response.thread.links.detail);

    Ok(())
}
