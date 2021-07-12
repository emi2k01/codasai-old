use serde::{Deserialize, Serialize};

use super::directory::VfsRoot;
use super::path::VfsPath;
use super::VfsFilesHandle;
use crate::vfs::VfsFile;
use crate::VfsWalker;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VfsSnapshot {
    #[serde(skip)]
    pub files: VfsFilesHandle,
    pub root: VfsRoot,
    pub page: String,
}

impl VfsSnapshot {
    pub fn new(files: VfsFilesHandle) -> Self {
        Self {
            files,
            root: VfsRoot::new(),
            page: String::new(),
        }
    }

    /// See [VfsRoot::create_directory](VfsRoot::create_directory)
    pub fn create_directory(&mut self, path: &VfsPath) {
        self.root.create_directory(path);
    }

    /// See [VfsRoot::delete_directory](VfsRoot::delete_directory)
    pub fn delete_directory(&mut self, path: &VfsPath) {
        self.root.delete_directory(path);
    }

    /// See [VfsRoot::rename_directory](VfsRoot::rename_directory)
    pub fn rename_directory(&mut self, old_path: &VfsPath, new_path: &VfsPath) {
        self.root.rename_directory(old_path, new_path);
    }

    /// Adds a file entry with the given `content` to the global array of files
    /// and creates a relationship between it and the given `path`.
    ///
    /// See [VfsRoot::create_file](VfsRoot::create_file)
    pub fn create_file(&mut self, path: &VfsPath, content: String) {
        let file_index = self.files.add_file(VfsFile::new(content));
        self.root.create_file(path, file_index);
    }

    /// Writes `content` into the file at the given `path` if it exists.
    pub fn write_file(&mut self, path: &VfsPath, content: String) {
        // for now it just forwards to `create_file` but later it will be used to record
        // the changes to each snapshot
        self.create_file(path, content);
    }

    /// Reads the file content at the given `path` and returns it.
    pub fn read_file(&self, path: &VfsPath) -> Option<String> {
        let file_index = self.root.find_file(path)?;
        self.files.read_file(file_index)
    }

    /// See [VfsRoot::delete_file](VfsRoot::delete_file)
    pub fn delete_file(&mut self, path: &VfsPath) {
        self.root.delete_file(path);
    }

    /// See [VfsRoot::rename_file](VfsRoot::rename_file)
    pub fn rename_file(&mut self, old_path: &VfsPath, new_path: &VfsPath) {
        self.root.rename_file(old_path, new_path);
    }

    pub fn set_page(&mut self, page: String) {
        self.page = page;
    }

    pub fn walk(&self) -> VfsWalker<'_> {
        VfsWalker::new(&self.root.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_file() -> anyhow::Result<()> {
        let mut snapshot = VfsSnapshot::new(VfsFilesHandle::new());
        snapshot.create_file(
            &VfsPath::new("/src/main.rs")?,
            "fn main() {\n    println!(\"Hello, world\");".into(),
        );

        insta::with_settings!({sort_maps => true}, {
            insta::assert_ron_snapshot!(snapshot, @r###"
            VfsSnapshot(
              root: VfsRoot(VfsDirectory(
                directories: {
                  "src": VfsDirectory(
                    directories: {},
                    files: {
                      "main.rs": 0,
                    },
                  ),
                },
                files: {},
              )),
              page: "",
            )
            "###);
        });

        Ok(())
    }

    #[test]
    fn test_write_read_file() -> anyhow::Result<()> {
        let mut snapshot = VfsSnapshot::new(VfsFilesHandle::new());

        let path = VfsPath::new("/src/main.rs")?;
        snapshot.create_file(&path, "fn main() {\n    println!(\"Hello, world\");".into());

        snapshot.write_file(&path, "fn main() {}".into());

        assert_eq!(snapshot.read_file(&path), Some("fn main() {}".into()));

        Ok(())
    }
}

impl PartialEq for VfsSnapshot {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root && self.page == other.page
    }
}
