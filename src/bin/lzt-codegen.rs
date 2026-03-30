#[path = "../codegen.rs"]
mod codegen;

use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("missing CARGO_MANIFEST_DIR"));
    let command = env::args().nth(1).unwrap_or_else(|| "generate".to_string());

    let result = match command.as_str() {
        "generate" => codegen::generate_workspace(&manifest_dir),
        "check" => codegen::check_workspace(&manifest_dir),
        other => Err(format!(
            "unknown command `{other}`. Expected `generate` or `check`."
        )),
    };

    if let Err(err) = result {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
