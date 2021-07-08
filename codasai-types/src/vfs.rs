use serde::{Deserialize, Serialize};

mod directory;
mod file;
mod path;
mod snapshot;
mod walker;

pub use directory::{VfsDirectory, VfsRoot};
pub use file::{VfsFile, VfsFilesHandle};
pub use path::VfsPath;
pub use snapshot::VfsSnapshot;
pub use walker::{VfsDirectoryOrFile, VfsWalker, VfsWalkerEntry};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vfs {
    pub files: VfsFilesHandle,
    pub snapshots: Vec<VfsSnapshot>,
}

impl Vfs {
    pub fn new() -> Self {
        Self {
            files: VfsFilesHandle::new(),
            snapshots: Vec::new(),
        }
    }

    pub fn propagate_files(&mut self) {
        for snapshot in &mut self.snapshots {
            snapshot.files = self.files.clone();
        }
    }

    pub fn add_snapshot(&mut self) -> &mut VfsSnapshot {
        let new_snapshot = if let Some(last_snapshot) = self.snapshots.last() {
            last_snapshot.clone()
        } else {
            VfsSnapshot::new(self.files.clone())
        };

        self.snapshots.push(new_snapshot);
        self.snapshots.last_mut().unwrap()
    }
}
