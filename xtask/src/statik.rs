use std::path::PathBuf;

use glob::glob;
use structopt::StructOpt;
use xshell::cmd;

use crate::util::{print, path};

static STATIC_FILES: &[&str] = &[
    "**/*.html",
    "**/*.css",
    "**/*.js",
    "**/*.jpg",
    "**/*.jpeg",
    "**/*.png",
    "**/*.svg",
];

static SASS_FILES: &str = "**/*.scss";

fn dist_dir() -> PathBuf {
    path::codasai_server_dir().join("dist")
}

fn static_dir() -> PathBuf {
    path::codasai_server_dir().join("static")
}

#[derive(Debug, StructOpt)]
pub struct Opts {
    #[structopt(long)]
    pub with_wasm_glue: bool,
}

pub fn run(opts: Opts) -> Result<(), Box<dyn std::error::Error>> {
    print::task("Running: Static");

    build_codasai_web()?;
    if opts.with_wasm_glue {
        wasm_bindgen_gen()?;
    }
    copy_static_files_to_dist()?;
    transpile_sass_files()?;

    Ok(())
}

fn build_codasai_web() -> Result<(), Box<dyn std::error::Error>> {
    print::step(format!("Building crate: {}", path::codasai_web_dir().display()));
    cmd!("cargo build --package codasai-web --target wasm32-unknown-unknown").run()?;

    Ok(())
}

fn wasm_bindgen_gen() -> Result<(), Box<dyn std::error::Error>> {
    print::step("wasm-bindgen: generating glue code");

    let wasm_file = path::project_root().join("target/wasm32-unknown-unknown/debug/codasai_web.wasm").to_string_lossy().into_owned();
    let dist_dir = dist_dir();

    if dist_dir.exists() {
        std::fs::remove_dir_all(&dist_dir)?;
    }

    cmd!("wasm-bindgen {wasm_file} --out-dir {dist_dir} --no-typescript --target web").run()?;

    Ok(())
}


fn transpile_sass_files() -> Result<(), Box<dyn std::error::Error>> {
    print::step("Transpiling sass files");

    let glob_lit = static_dir().join(SASS_FILES).display().to_string();
    let sass_files = glob(&glob_lit).unwrap().map(Result::unwrap);

    for sass_file_path in sass_files {
        let relative_path = sass_file_path.strip_prefix(static_dir())?;

        let css_path = relative_path.with_extension("css");

        let dist_path = dist_dir().join(css_path).display().to_string();
        let sass_file_path = sass_file_path.display().to_string();

        cmd!("sass {sass_file_path} {dist_path}").run()?;
    }

    Ok(())
}

fn copy_static_files_to_dist() -> Result<(), Box<dyn std::error::Error>> {
    let matched_static_files = STATIC_FILES.into_iter()
        .flat_map(|p| glob(&static_dir().join(*p).display().to_string()).expect("a valid glob"))
        .map(|p| p.unwrap());

    for file in matched_static_files {
        let relative_path = file.strip_prefix(static_dir())?;

        let dist_path = dist_dir().join(relative_path).display().to_string();
        let static_path = file.display().to_string();

        print::step(format!("Copying file: {} -> {}", static_path, dist_path));
        xshell::mkdir_p(dist_dir().parent().unwrap())?;
        xshell::cp(&static_path, &dist_path)?;
    }

    Ok(())
}
