
pub mod path {
    use anyhow::{anyhow, Result, Context};
    use std::path::{Path, PathBuf};
    use std::fs;

    pub fn dotcodasai() -> Result<PathBuf> {
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
}
