use structopt::StructOpt;

use crate::{statik, util::print};

#[derive(Debug, StructOpt)]
pub struct Opts {}

pub fn run(_opts: Opts) -> Result<(), Box<dyn std::error::Error>> {
    print::task("Running: Build");
    statik::run(statik::Opts { with_wasm_glue: true })?;

    print::step("Building Codasai");
    xshell::cmd!("cargo build").run()?;

    Ok(())
}
