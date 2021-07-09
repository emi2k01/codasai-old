use std::env;

fn main() {
    if is_static_dir_empty().unwrap_or(true) {
        println!(
            "cargo:warning=Run `cargo xtask setup` before compiling this crate or build \
             with `cargo xtask build`."
        );
    }
}

fn is_static_dir_empty() -> Result<bool, Box<dyn std::error::Error>> {
    const CODASAI_SERVER_STATIC_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/static");

    Ok(std::fs::read_dir(CODASAI_SERVER_STATIC_PATH)?.count() == 0)
}
