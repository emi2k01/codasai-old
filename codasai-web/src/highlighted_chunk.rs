use std::ops::Range;

use anyhow::{anyhow, Result};
use codasai_types::VfsPath;

#[derive(Debug, Clone, PartialEq)]
pub struct HighlightedChunk {
    pub file: VfsPath,
    pub line_range: Range<usize>,
    pub class: String,
}

impl HighlightedChunk {
    /// Parse chunk from `data-rel` attribute.
    ///
    /// The syntax of `data-rel` is `$path:$line_start..$line_end`
    pub fn from_data_rel(attr: &str, class: String) -> Result<HighlightedChunk> {
        let mut components = attr.split(":");

        let file = components
            .next()
            .ok_or_else(|| anyhow!("file not present in `data-rel`"))?;
        let file = VfsPath::new(&file)?;

        let mut lines_component = components
            .next()
            .ok_or_else(|| anyhow!("line range not present in data-rel"))?
            .split("..");

        let line_start = lines_component
            .next()
            .and_then(|n| n.parse().ok())
            .ok_or_else(|| anyhow!("line start in data-rel is not valid"))?;

        let line_end = lines_component
            .next()
            .and_then(|n| n.parse().ok())
            .ok_or_else(|| anyhow!("line end in data-rel is not valid"))?;

        Ok(Self {
            file,
            line_range: line_start..line_end,
            class,
        })
    }
}
