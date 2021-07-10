use structopt::StructOpt;

mod build;
mod statik;
mod util;

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(subcommand)]
    subcmd: SubCmd,
}

#[derive(Debug, StructOpt)]
enum SubCmd {
    Static(statik::Opts),
    Build(build::Opts),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::from_args();

    match opts.subcmd {
        SubCmd::Static(opts) => statik::run(opts)?,
        SubCmd::Build(opts) => build::run(opts)?,
    }

    Ok(())
}
