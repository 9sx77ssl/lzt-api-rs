use lzt_api::LolzteamClient;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("LZT_API_TOKEN").unwrap_or_else(|_| "your_token".to_string());

    let client = LolzteamClient::new(&token);

    println!("=== Forum API ===\n");

    println!("Categories:");
    let categories = client.forum().categories_list(Default::default()).await?;
    println!("  Total: {}", categories.categories_total);

    println!("\nForums:");
    let forums = client.forum().forums_list(Default::default()).await?;
    println!("  Total: {}", forums.forums_total);

    println!("\nUser:");
    let user = client
        .forum()
        .users_get(json!(1), Default::default())
        .await?;
    println!("  Response: {:?}", user);

    Ok(())
}
