use serde_json::Value;
use std::fs;
use std::path::Path;

fn count_operations(schema_path: &Path) -> usize {
    let raw = fs::read_to_string(schema_path).expect("schema should be readable");
    let schema: Value = serde_json::from_str(&raw).expect("schema should be valid json");
    schema["paths"]
        .as_object()
        .expect("paths should be an object")
        .values()
        .map(|path_item| {
            path_item
                .as_object()
                .expect("path item should be an object")
                .keys()
                .filter(|method| {
                    matches!(method.as_str(), "get" | "post" | "put" | "delete" | "patch")
                })
                .count()
        })
        .sum()
}

fn count_generated_methods(source_path: &Path) -> usize {
    fs::read_to_string(source_path)
        .expect("methods file should be readable")
        .matches("pub async fn ")
        .count()
}

#[test]
fn generated_forum_methods_match_openapi_operations() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert_eq!(
        count_operations(&root.join("forum.json")),
        count_generated_methods(&root.join("src/forum/methods.rs"))
    );
}

#[test]
fn generated_market_methods_match_openapi_operations() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert_eq!(
        count_operations(&root.join("market.json")),
        count_generated_methods(&root.join("src/market/methods.rs"))
    );
}
