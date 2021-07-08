use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::Vfs;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Guide {
    pub name: String,
    pub vfs: Vfs,
}

impl Guide {
    pub fn new(name: String, vfs: Vfs) -> Self {
        Self { name, vfs }
    }

    pub fn from_json(s: &str) -> Result<Self> {
        let mut this: Self = serde_json::from_str(s)?;
        this.vfs.propagate_files();
        Ok(this)
    }
}
