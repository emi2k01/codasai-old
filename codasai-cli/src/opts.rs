use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Codasai", about = "Codasai CLI help")]
pub enum CliOpts {
    /// Initializes the guide in the current directory. The current directory
    /// must be empty.
    Init(InitOpts),

    /// Builds the guide and outputs a JSON file that can be open with the web
    /// app.
    Build(BuildOpts),

    Page(PageOpts),

    Serve(ServeOpts),
}

#[derive(Debug, StructOpt)]
pub struct InitOpts {
    pub title: String,
}

#[derive(Debug, StructOpt)]
pub struct BuildOpts {
    #[structopt(default_value = ".")]
    pub guide: PathBuf,
}

#[derive(Debug, StructOpt)]
pub struct PageOpts {
    #[structopt(subcommand)]
    pub subcmd: PageSubcmd,
}

#[derive(Debug, StructOpt)]
pub enum PageSubcmd {
    New(PageNewOpts),
    Save(PageSaveOpts),
}

#[derive(Debug, StructOpt)]
pub struct PageNewOpts {
    pub title: String,
}

#[derive(Debug, StructOpt)]
pub struct PageSaveOpts {
    #[structopt(short, long)]
    pub message: Option<String>,
}

#[derive(Debug, StructOpt)]
pub struct ServeOpts {}

impl CliOpts {
    pub fn from_args() -> Self {
        StructOpt::from_args()
    }
}
