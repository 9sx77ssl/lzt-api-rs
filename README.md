# lzt-api

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![CI](https://github.com/9sx77ssl/lzt-api-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/9sx77ssl/lzt-api-rs/actions/workflows/ci.yml)

Rust API wrapper for LOLZTEAM Forum and Market generated from the official `forum.json` and `market.json` OpenAPI schemas.

## What this repo now guarantees

- Forum and Market clients are generated from OpenAPI `operationId` + schema shape.
- Generated request and response types live in tracked source files and are reproducible.
- Proxy support is available for both APIs together or separately.
- Authenticated `http://`, `https://`, and `socks5://` proxies are supported.
- Automatic retry is implemented for `429`, `502`, and `503`.
- MIT license and GitHub Actions are ready for handoff and publication.

## Install

```toml
[dependencies]
lzt-api = { git = "https://github.com/noki44ngel/lzt-api-rs.git" }
tokio = { version = "1", features = ["full"] }
serde_json = "1"
```

## Quick start

```rust
use lzt_api::LolzteamClient;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LolzteamClient::new("YOUR_TOKEN");

    let categories = client
        .forum()
        .categories_list(Default::default())
        .await?;
    println!("Forum categories: {}", categories.categories_total);

    let user = client
        .forum()
        .users_get(json!(1), Default::default())
        .await?;
    println!("User payload: {:?}", user);

    let steam = client.market().category_steam(Default::default()).await?;
    println!("Steam items: {}", steam.total_items);

    Ok(())
}
```

## Builder, proxy, retry

```rust
use lzt_api::LolzteamClient;
use std::time::Duration;

let client = LolzteamClient::builder("YOUR_TOKEN")
    .forum_proxy("socks5://user:password@127.0.0.1:1080")
    .market_proxy("http://user:password@127.0.0.1:8080")
    .timeout(Duration::from_secs(30))
    .max_retries(5)
    .build()?;
```

Retry policy:

- retries `429`, `502`, `503`
- respects `Retry-After` for `429`
- uses exponential backoff starting at 2 seconds

## Code generation

`forum.json` and `market.json` are the only sources of truth. Generated files are:

- [src/forum/methods.rs](/home/rsz/Desktop/lzt-api-rs/src/forum/methods.rs)
- [src/forum/types.rs](/home/rsz/Desktop/lzt-api-rs/src/forum/types.rs)
- [src/market/methods.rs](/home/rsz/Desktop/lzt-api-rs/src/market/methods.rs)
- [src/market/types.rs](/home/rsz/Desktop/lzt-api-rs/src/market/types.rs)

Regenerate:

```bash
cargo run --bin lzt-codegen -- generate
```

Check that generated code is up to date:

```bash
cargo run --bin lzt-codegen -- check
```

The build script also refreshes generated files automatically when the schemas change.

Runnable example for creating a forum thread:

```bash
cargo run --example create_thread
```

## Tests

Local checks:

```bash
cargo run --bin lzt-codegen -- check
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test
cargo doc --no-deps
```

Test coverage currently includes:

- generated method count parity against OpenAPI
- request wire-shape checks with `wiremock`
- error handling for `401`, `403`, `429`, `502`, `503`
- proxy builder validation including authenticated `socks5://` URLs
- compile and doctest coverage for examples/docs

Live API smoke testing is separated into `.github/workflows/live-smoke.yml` and requires `LZT_API_TOKEN`.

## Release flow

The main CI workflow:

- verifies generated code is current
- runs fmt, clippy, tests, docs, release build, and `cargo package`

The release job is set up for `cargo publish`, so the repository can be published later from the maintainers' account.

## License

MIT. See [LICENSE](/home/rsz/Desktop/lzt-api-rs/LICENSE).
