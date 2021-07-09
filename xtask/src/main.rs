use structopt::StructOpt;

mod build;
mod setup;
mod util;

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(subcommand)]
    subcmd: SubCmd,
}

#[derive(Debug, StructOpt)]
enum SubCmd {
    Setup(setup::Opts),
    Build(build::Opts),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::from_args();

    match opts.subcmd {
        SubCmd::Setup(opts) => setup::run(opts)?,
        SubCmd::Build(opts) => build::run(opts)?,
    }

    Ok(())
}
