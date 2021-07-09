use structopt::StructOpt;

use crate::util::print;
use crate::setup;

#[derive(Debug, StructOpt)]
pub struct Opts {}

pub fn run(_opts: Opts) -> Result<(), Box<dyn std::error::Error>> {
    print::task("Running: Build");
    setup::run(setup::Opts {})?;

    print::step("Building Codasai");
    xshell::cmd!("cargo build").run()?;

    Ok(())
}
