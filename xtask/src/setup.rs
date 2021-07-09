use fs_extra::dir::CopyOptions;
use structopt::StructOpt;
use xshell::cmd;

use crate::util::{print, path};

#[derive(Debug, StructOpt)]
pub struct Opts {}

pub fn run(_opts: Opts) -> Result<(), Box<dyn std::error::Error>> {
    print::task("Running: Setup");

    print::step(format!("Building crate: {}", path::codasai_web_dir().display()));
    cmd!("cargo build --package codasai-web").run()?;

    let from = path::codasai_web_dir().join("dist");
    let to = path::codasai_server_dir().join("static");

    print::step(format!("Copying static files: from `{}` to `{}`", from.display(), to.display()));
    if to.exists() {
        std::fs::remove_dir_all(&to)?;
    }

    fs_extra::copy_items(&[&from], &to, &CopyOptions {
        overwrite: true,
        copy_inside: true,
        ..Default::default()
    })?;

    Ok(())
}
