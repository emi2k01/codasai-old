use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VfsFilesHandle(Rc<RefCell<Vec<VfsFile>>>);

impl VfsFilesHandle {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(Vec::new())))
    }

    pub fn add_file(&self, file: VfsFile) -> usize {
        let mut files = self.0.borrow_mut();
        files.push(file);

        files.len() - 1
    }

    pub fn write_file(&self, file_index: usize, content: String) {
        if let Some(file) = self.0.borrow_mut().get_mut(file_index) {
            file.content = content;
        } else {
            tracing::warn!("tried to write into a non-existing file");
        }
    }

    pub fn read_file(&self, file_index: usize) -> Option<String> {
        self.0.borrow().get(file_index).map(|f| f.content.clone())
    }
}

impl Default for VfsFilesHandle {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VfsFile {
    content: String,
}

impl VfsFile {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}
