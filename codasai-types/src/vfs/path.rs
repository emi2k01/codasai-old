use std::convert::{TryFrom, TryInto};
use std::fmt::Display;
use std::path::{Component, Path, PathBuf};

use anyhow::{ensure, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VfsPath(PathBuf);

impl VfsPath {
    pub fn new(path: impl AsRef<Path>) -> Result<VfsPath> {
        let is_valid = path
            .as_ref()
            .components()
            .all(|c| matches!(c, Component::RootDir | Component::Normal(_)));

        ensure!(is_valid, "path must be normalized");

        let path = path
            .as_ref()
            .components()
            // Don't collect root slash (`/`)
            .filter(|c| matches!(c, Component::Normal(_)))
            .collect();

        Ok(Self(path))
    }

    pub fn parent(&self) -> VfsPath {
        self.0.parent().unwrap().try_into().unwrap()
    }

    pub fn file_name(&self) -> &str {
        self.0.file_name().unwrap().to_str().unwrap()
    }

    pub fn components(&self) -> impl Iterator<Item = &str> {
        self.0.components().filter_map(|c| {
            if let Component::Normal(n) = c {
                Some(
                    n.to_str()
                        .ok_or_else(|| {
                            unreachable!(
                                "path encoding should have been checked at construction time"
                            )
                        })
                        .unwrap(),
                )
            } else {
                unreachable!("path should be normalized at construction time")
            }
        })
    }

    pub fn into_string(self) -> String {
        self.0.into_os_string().into_string().unwrap()
    }

    pub fn as_str(&self) -> &str {
        self.0.as_os_str().to_str().unwrap()
    }
}

impl Display for VfsPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<PathBuf> for VfsPath {
    type Error = anyhow::Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&Path> for VfsPath {
    type Error = anyhow::Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        Self::new(value.to_path_buf())
    }
}
