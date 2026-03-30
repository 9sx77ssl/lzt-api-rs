use lzt_api::forum::{CategoriesListQuery, ChatboxDeleteIgnoreRequest};
use lzt_api::{Error, LolzteamClient};
use serde_json::json;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn forum_client(base_url: &str) -> LolzteamClient {
    LolzteamClient::builder("test-token")
        .forum_base_url(base_url)
        .market_base_url(base_url)
        .max_retries(2)
        .build()
        .expect("client should build")
}

fn categories_payload() -> serde_json::Value {
    json!({
        "categories": [],
        "categories_total": 0,
        "system_info": {
            "time": 1,
            "visitor_id": 2
        }
    })
}

#[tokio::test]
async fn forum_query_params_follow_generated_shape() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/categories"))
        .and(query_param("order", "desc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(categories_payload()))
        .mount(&server)
        .await;

    let client = forum_client(&server.uri());
    let response = client
        .forum()
        .categories_list(CategoriesListQuery {
            order: Some("desc".to_string()),
            ..Default::default()
        })
        .await
        .expect("request should succeed");

    assert_eq!(response.categories_total, 0);
}

#[tokio::test]
async fn forum_json_body_is_sent_for_delete_requests() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/chatbox/ignore"))
        .and(body_json(json!({ "user_id": 1 })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!(null)))
        .mount(&server)
        .await;

    let client = forum_client(&server.uri());
    client
        .forum()
        .chatbox_delete_ignore(ChatboxDeleteIgnoreRequest { user_id: json!(1) })
        .await
        .expect("delete request should serialize the body");
}

#[tokio::test]
async fn retry_policy_retries_429_and_respects_retry_after() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/categories"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("retry-after", "0")
                .set_body_string("rate limited"),
        )
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/categories"))
        .respond_with(ResponseTemplate::new(200).set_body_json(categories_payload()))
        .mount(&server)
        .await;

    let client = forum_client(&server.uri());
    let response = client
        .forum()
        .categories_list(Default::default())
        .await
        .expect("client should retry and eventually succeed");

    assert_eq!(response.system_info.time, 1);
}

#[tokio::test]
async fn unauthorized_errors_keep_status_and_json_body() {
    let server = MockServer::start().await;
    let body = json!({
        "errors": ["Unauthorized"],
        "system_info": {
            "time": 1,
            "visitor_id": 2
        }
    });
    Mock::given(method("GET"))
        .and(path("/categories"))
        .respond_with(ResponseTemplate::new(401).set_body_json(&body))
        .mount(&server)
        .await;

    let client = forum_client(&server.uri());
    let error = client
        .forum()
        .categories_list(Default::default())
        .await
        .expect_err("request should fail");

    match error {
        Error::Api { status, body } => {
            assert_eq!(status, 401);
            assert!(body.contains("Unauthorized"));
            assert!(body.contains("system_info"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[tokio::test]
async fn forbidden_errors_keep_status_and_json_body() {
    let server = MockServer::start().await;
    let body = json!({
        "errors": ["Need to wait before posting again"],
        "system_info": {
            "time": 1,
            "visitor_id": 2
        }
    });
    Mock::given(method("GET"))
        .and(path("/categories"))
        .respond_with(ResponseTemplate::new(403).set_body_json(&body))
        .mount(&server)
        .await;

    let client = forum_client(&server.uri());
    let error = client
        .forum()
        .categories_list(Default::default())
        .await
        .expect_err("request should fail");

    match error {
        Error::Api { status, body } => {
            assert_eq!(status, 403);
            assert!(body.contains("Need to wait before posting again"));
            assert!(body.contains("system_info"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[tokio::test]
async fn exhausted_429_keeps_retry_after_and_body() {
    let server = MockServer::start().await;
    let body = json!({
        "errors": ["Too many requests"],
        "system_info": {
            "time": 1,
            "visitor_id": 2
        }
    });
    Mock::given(method("GET"))
        .and(path("/categories"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("retry-after", "0")
                .set_body_json(&body),
        )
        .mount(&server)
        .await;

    let client = LolzteamClient::builder("test-token")
        .forum_base_url(server.uri())
        .market_base_url(server.uri())
        .max_retries(0)
        .build()
        .expect("client should build");

    let error = client
        .forum()
        .categories_list(Default::default())
        .await
        .expect_err("request should fail");

    match error {
        Error::RateLimited {
            attempts,
            retry_after,
            body,
        } => {
            assert_eq!(attempts, 1);
            assert_eq!(retry_after, Some(0));
            assert!(body.contains("Too many requests"));
            assert!(body.contains("system_info"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[tokio::test]
async fn exhausted_502_keeps_status_and_body() {
    let server = MockServer::start().await;
    let body = json!({
        "errors": ["Bad gateway"],
        "system_info": {
            "time": 1,
            "visitor_id": 2
        }
    });
    Mock::given(method("GET"))
        .and(path("/categories"))
        .respond_with(ResponseTemplate::new(502).set_body_json(&body))
        .mount(&server)
        .await;

    let client = LolzteamClient::builder("test-token")
        .forum_base_url(server.uri())
        .market_base_url(server.uri())
        .max_retries(0)
        .build()
        .expect("client should build");

    let error = client
        .forum()
        .categories_list(Default::default())
        .await
        .expect_err("request should fail");

    match error {
        Error::RetryExhausted {
            status,
            attempts,
            body,
        } => {
            assert_eq!(status, 502);
            assert_eq!(attempts, 1);
            assert!(body.contains("Bad gateway"));
            assert!(body.contains("system_info"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn builder_accepts_authenticated_socks5_proxy_urls() {
    let client = LolzteamClient::builder("test-token")
        .proxy("socks5://user:password@127.0.0.1:1080")
        .build();

    assert!(
        client.is_ok(),
        "authenticated socks5 proxy should be accepted"
    );
}

#[tokio::test]
async fn exhausted_503_keeps_status_and_body() {
    let server = MockServer::start().await;
    let body = json!({
        "errors": ["Service unavailable"],
        "system_info": {
            "time": 1,
            "visitor_id": 2
        }
    });
    Mock::given(method("GET"))
        .and(path("/categories"))
        .respond_with(ResponseTemplate::new(503).set_body_json(&body))
        .mount(&server)
        .await;

    let client = LolzteamClient::builder("test-token")
        .forum_base_url(server.uri())
        .market_base_url(server.uri())
        .max_retries(0)
        .build()
        .expect("client should build");

    let error = client
        .forum()
        .categories_list(Default::default())
        .await
        .expect_err("request should fail");

    match error {
        Error::RetryExhausted {
            status,
            attempts,
            body,
        } => {
            assert_eq!(status, 503);
            assert_eq!(attempts, 1);
            assert!(body.contains("Service unavailable"));
            assert!(body.contains("system_info"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn builder_rejects_invalid_proxy_urls() {
    let client = LolzteamClient::builder("test-token").proxy("://").build();

    assert!(matches!(client, Err(Error::InvalidProxy(_))));
}
