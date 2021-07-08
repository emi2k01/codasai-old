use std::env;

fn main() {
    if is_static_dir_empty().unwrap_or(true) {
        println!(
            "cargo:warning=Run `just codasai-server/setup` before compiling this crate or build \
             with `just build`.\nThis assumes you're in the root directory of the project."
        );
        std::process::exit(1);
    }
}

fn is_static_dir_empty() -> Result<bool, Box<dyn std::error::Error>> {
    const CODASAI_SERVER_STATIC_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/static");

    Ok(std::fs::read_dir(CODASAI_SERVER_STATIC_PATH)?.count() == 0)
}
