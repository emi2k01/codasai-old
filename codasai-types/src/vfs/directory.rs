use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::path::VfsPath;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VfsRoot(pub(super) VfsDirectory);

impl VfsRoot {
    pub fn new() -> Self {
        Self(VfsDirectory::new())
    }

    /// Creates the directory at the given path, creating any ancestor that does
    /// not exist.
    pub fn create_directory(&mut self, path: &VfsPath) {
        find_or_create_directory_mut(&mut self.0, path);
    }

    /// Deletes a directory if it exists. If the directory does not exist, it
    /// won't do anything.
    pub fn delete_directory(&mut self, path: &VfsPath) {
        let parent_directory =
            if let Some(directory) = find_directory_mut(&mut self.0, &path.parent()) {
                directory
            } else {
                return;
            };

        if parent_directory
            .directories
            .remove(path.file_name())
            .is_none()
        {
            tracing::warn!("tried to delete a non-existing directory");
        }
    }

    /// Renames a directory given by `old_path` making its path match
    /// `new_path`. That is, this will move the directory if it's necessary
    /// to match `new_path`, replacing the directory in `new_path` if
    /// there's already one.
    pub fn rename_directory(&mut self, old_path: &VfsPath, new_path: &VfsPath) {
        let old_directory_parent =
            if let Some(directory) = find_directory_mut(&mut self.0, &old_path.parent()) {
                directory
            } else {
                return;
            };

        let old_directory = if let Some(old_directory) = old_directory_parent
            .directories
            .remove(old_path.file_name())
        {
            old_directory
        } else {
            tracing::warn!("tried to rename non-existing directory");
            return;
        };

        let new_directory_parent = find_or_create_directory_mut(&mut self.0, &new_path.parent());

        new_directory_parent
            .directories
            .insert(new_path.file_name().to_string(), old_directory);
    }

    /// Creates the file at the given path, creating any ancestor that does not
    /// exist.
    pub fn create_file(&mut self, path: &VfsPath, file_index: usize) {
        let parent_directory = find_or_create_directory_mut(&mut self.0, &path.parent());

        parent_directory.create_file(path.file_name().to_string(), file_index);
    }

    /// Deletes a file if it exists. If the file does not exist, it won't do
    /// anything.
    pub fn delete_file(&mut self, path: &VfsPath) {
        let parent_directory =
            if let Some(directory) = find_directory_mut(&mut self.0, &path.parent()) {
                directory
            } else {
                return;
            };

        parent_directory.files.remove(path.file_name());
    }

    /// Renames a file given by `old_path` making its path match `new_path`.
    /// That is, this will move the file if it's necessary to match
    /// `new_path`, replacing the file in `new_path` if there's already one.
    pub fn rename_file(&mut self, old_path: &VfsPath, new_path: &VfsPath) {
        let old_file_parent =
            if let Some(directory) = find_directory_mut(&mut self.0, &old_path.parent()) {
                directory
            } else {
                return;
            };

        let old_file = if let Some(old_file) = old_file_parent.files.remove(old_path.file_name()) {
            old_file
        } else {
            tracing::warn!("tried to rename non-existing file");
            return;
        };

        let new_file_parent = find_or_create_directory_mut(&mut self.0, &new_path.parent());

        new_file_parent
            .files
            .insert(new_path.file_name().to_string(), old_file);
    }

    pub fn find_file(&self, path: &VfsPath) -> Option<usize> {
        let mut current_directory = &self.0;
        for component in path.parent().components() {
            if let Some(child_directory) = current_directory.directory(component) {
                current_directory = child_directory;
            } else {
                return None;
            }
        }

        current_directory.files.get(path.file_name()).copied()
    }
}

fn find_or_create_directory_mut<'a>(
    root: &'a mut VfsDirectory, path: &VfsPath,
) -> &'a mut VfsDirectory {
    let mut current_directory = root;
    for component in path.components() {
        current_directory = current_directory.get_or_create_directory(component.to_string());
    }
    current_directory
}

fn find_directory_mut<'a>(
    root: &'a mut VfsDirectory, path: &VfsPath,
) -> Option<&'a mut VfsDirectory> {
    let mut current_directory = root;
    for component in path.components() {
        if let Some(child_directory) = current_directory.directory_mut(component) {
            current_directory = child_directory;
        } else {
            return None;
        }
    }

    Some(current_directory)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VfsDirectory {
    pub(super) directories: BTreeMap<String, VfsDirectory>,
    pub(super) files: BTreeMap<String, usize>,
}

impl VfsDirectory {
    pub fn new() -> Self {
        Self {
            directories: BTreeMap::new(),
            files: BTreeMap::new(),
        }
    }

    #[allow(unused)]
    fn create_directory(&mut self, name: String) {
        self.directories.insert(name, VfsDirectory::new());
    }

    fn create_file(&mut self, name: String, file_index: usize) {
        self.files.insert(name, file_index);
    }

    #[allow(unused)]
    fn contains_directory(&self, name: &str) -> bool {
        self.directories.contains_key(name)
    }

    fn directory(&self, name: &str) -> Option<&VfsDirectory> {
        self.directories.get(name)
    }

    fn directory_mut(&mut self, name: &str) -> Option<&mut VfsDirectory> {
        self.directories.get_mut(name)
    }

    fn get_or_create_directory(&mut self, name: String) -> &mut VfsDirectory {
        self.directories
            .entry(name)
            .or_insert_with(VfsDirectory::new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_create_directory() -> anyhow::Result<()> {
        let mut root = VfsRoot::new();
        root.create_directory(&VfsPath::new("src/models/db")?);

        insta::with_settings!({sort_maps => true}, {
            insta::assert_ron_snapshot!(root, @r###"
            VfsRoot(VfsDirectory(
              directories: {
                "src": VfsDirectory(
                  directories: {
                    "models": VfsDirectory(
                      directories: {
                        "db": VfsDirectory(
                          directories: {},
                          files: {},
                        ),
                      },
                      files: {},
                    ),
                  },
                  files: {},
                ),
              },
              files: {},
            ))
            "###);
        });

        Ok(())
    }

    #[test]
    fn test_root_delete_directory() -> anyhow::Result<()> {
        let mut root = VfsRoot::new();

        let directory_path = VfsPath::new("src/models/db")?;
        root.create_directory(&directory_path);
        root.delete_directory(&directory_path);

        insta::with_settings!({sort_maps => true}, {
            insta::assert_ron_snapshot!(root, @r###"
            VfsRoot(VfsDirectory(
              directories: {
                "src": VfsDirectory(
                  directories: {
                    "models": VfsDirectory(
                      directories: {},
                      files: {},
                    ),
                  },
                  files: {},
                ),
              },
              files: {},
            ))
            "###);
        });

        Ok(())
    }

    #[test]
    fn test_root_rename_directory() -> anyhow::Result<()> {
        let mut root = VfsRoot::new();

        root.create_directory(&VfsPath::new("/src/math/f64")?);
        root.rename_directory(
            &VfsPath::new("/src/math/")?,
            &VfsPath::new("/src/mathematics")?,
        );
        root.rename_directory(
            &VfsPath::new("/src/mathematics/f64")?,
            &VfsPath::new("/src/mathematics/floating64")?,
        );

        insta::with_settings!({sort_maps => true}, {
            insta::assert_ron_snapshot!(root, @r###"
            VfsRoot(VfsDirectory(
              directories: {
                "src": VfsDirectory(
                  directories: {
                    "mathematics": VfsDirectory(
                      directories: {
                        "floating64": VfsDirectory(
                          directories: {},
                          files: {},
                        ),
                      },
                      files: {},
                    ),
                  },
                  files: {},
                ),
              },
              files: {},
            ))
            "###);
        });

        Ok(())
    }

    #[test]
    fn test_root_create_file() -> anyhow::Result<()> {
        let mut root = VfsRoot::new();

        let file_path = VfsPath::new("/docs/README.md")?;
        root.create_file(&file_path, 0);

        insta::with_settings!({sort_maps => true}, {
            insta::assert_ron_snapshot!(root, @r###"
            VfsRoot(VfsDirectory(
              directories: {
                "docs": VfsDirectory(
                  directories: {},
                  files: {
                    "README.md": 0,
                  },
                ),
              },
              files: {},
            ))
            "###);
        });

        Ok(())
    }

    #[test]
    fn test_root_delete_file() -> anyhow::Result<()> {
        let mut root = VfsRoot::new();

        let file_path = VfsPath::new("/docs/README.md")?;
        root.create_file(&file_path, 0);
        root.delete_file(&file_path);

        insta::with_settings!({sort_maps => true}, {
            insta::assert_ron_snapshot!(root, @r###"
            VfsRoot(VfsDirectory(
              directories: {
                "docs": VfsDirectory(
                  directories: {},
                  files: {},
                ),
              },
              files: {},
            ))
            "###);
        });

        Ok(())
    }

    #[test]
    fn test_root_rename_file() -> anyhow::Result<()> {
        let mut root = VfsRoot::new();

        let old_file_path = VfsPath::new("/docs/README.md")?;
        let new_file_path = VfsPath::new("/dev/docs/SUMMARY.md")?;
        root.create_file(&old_file_path, 0);
        root.rename_file(&old_file_path, &new_file_path);

        insta::with_settings!({sort_maps => true}, {
            insta::assert_ron_snapshot!(root, @r###"
            VfsRoot(VfsDirectory(
              directories: {
                "dev": VfsDirectory(
                  directories: {
                    "docs": VfsDirectory(
                      directories: {},
                      files: {
                        "SUMMARY.md": 0,
                      },
                    ),
                  },
                  files: {},
                ),
                "docs": VfsDirectory(
                  directories: {},
                  files: {},
                ),
              },
              files: {},
            ))
            "###);
        });

        Ok(())
    }

    #[test]
    fn test_root_find_file() -> anyhow::Result<()> {
        let mut root = VfsRoot::new();

        let path = VfsPath::new("/home/BarryBenson/porn/vannessa/1.jpg")?;
        root.create_file(&path, 15);
        assert_eq!(root.find_file(&path), Some(15));

        Ok(())
    }
}
