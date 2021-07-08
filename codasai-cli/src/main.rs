use anyhow::Result;

use crate::opts::CliOpts;

mod commands;
mod config;
mod opts;

fn main() -> Result<()> {
    let opts = CliOpts::from_args();

    match opts {
        CliOpts::Init(opts) => commands::init(&opts)?,
        CliOpts::Build(opts) => commands::build(&opts)?,
        CliOpts::Serve(opts) => commands::serve(&opts)?,
        CliOpts::Page(opts) => commands::page(&opts)?,
    }

    Ok(())
}
