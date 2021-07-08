use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use codasai_server::{Server, SharedState};

use crate::opts::ServeOpts;

pub fn serve(_opts: &ServeOpts) -> Result<()> {
    let dotcodasai = find_dotcodasai()?;
    let guide_json = find_guide_json(&dotcodasai)?;
    Server::new(SharedState::new(guide_json)).launch();

    Ok(())
}

fn find_dotcodasai() -> Result<PathBuf> {
    let mut this_dir = Path::new(".")
        .canonicalize()
        .with_context(|| "failed to canonicalize current directory")?;
    let mut current_dir = fs::read_dir(&this_dir)
        .with_context(|| format!("failed to read directory {:?}", this_dir))?;
    let mut dotcodasai = None;

    loop {
        for entry in current_dir.filter_map(Result::ok) {
            if entry.file_name() == ".codasai" && matches!(entry.file_type(), Ok(f) if f.is_dir()) {
                dotcodasai = Some(entry.path());
                break;
            }
        }

        if let Some(this_dir_parent) = this_dir.parent() {
            this_dir = this_dir_parent.to_path_buf();
        } else {
            break;
        }

        current_dir = fs::read_dir(&this_dir)
            .with_context(|| format!("failed to read directory {:?}", this_dir))?;
    }

    dotcodasai.ok_or_else(|| anyhow!("failed to find \".codasai\" directory"))
}

fn find_guide_json(dotcodasai: &Path) -> Result<String> {
    let guide_json_path = dotcodasai.join("out/guide.json");
    fs::read_to_string(&guide_json_path)
        .with_context(|| format!("failed to read guide.json at {:?}", guide_json_path))
}
