#[path = "src/codegen.rs"]
mod codegen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=forum.json");
    println!("cargo:rerun-if-changed=market.json");
    println!("cargo:rerun-if-changed=src/codegen.rs");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("missing manifest dir"));
    if let Err(err) = codegen::generate_workspace(&manifest_dir) {
        panic!("code generation failed: {err}");
    }
}
