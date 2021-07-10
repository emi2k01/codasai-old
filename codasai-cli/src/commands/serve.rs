use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use codasai_server::{Server, SharedState};

use crate::opts::ServeOpts;
use crate::util::path;

pub fn serve(_opts: &ServeOpts) -> Result<()> {
    let dotcodasai = path::dotcodasai()?;
    let guide_json = find_guide_json(&dotcodasai)?;
    Server::new(SharedState::new(guide_json)).launch();

    Ok(())
}

fn find_guide_json(dotcodasai: impl AsRef<Path>) -> Result<String> {
    let guide_json_path = dotcodasai.as_ref().join("out/guide.json");
    fs::read_to_string(&guide_json_path)
        .with_context(|| format!("failed to read guide.json at {:?}", guide_json_path))
}
