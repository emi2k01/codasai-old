use std::collections::btree_map;

use crate::{VfsDirectory, VfsPath};

#[derive(Debug, Clone)]
pub struct VfsWalker<'a> {
    level: usize,
    stack: Vec<WalkerLevel<'a>>,
    path_stack: Vec<&'a str>,
}

impl<'a> VfsWalker<'a> {
    pub fn new(contents: &'a VfsDirectory) -> Self {
        Self {
            level: 1,
            stack: vec![WalkerLevel {
                directories: contents.directories.iter(),
                files: contents.files.iter(),
            }],
            path_stack: Vec::new(),
        }
    }
}

impl<'a> Iterator for VfsWalker<'a> {
    type Item = VfsWalkerEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current_directory) = self.stack.last_mut() {
            if let Some((name, contents)) = current_directory.directories.next() {
                self.stack.push(WalkerLevel {
                    directories: contents.directories.iter(),
                    files: contents.files.iter(),
                });

                self.path_stack.push(name);
                self.level += 1;

                let path = VfsPath::new(self.path_stack.join("/")).unwrap();
                return Some(VfsWalkerEntry::new(
                    path,
                    self.level - 1,
                    VfsDirectoryOrFile::Directory(name),
                ));
            }

            if let Some((name, _)) = current_directory.files.next() {
                let path = VfsPath::new(format!("{}/{}", self.path_stack.join("/"), name)).unwrap();

                return Some(VfsWalkerEntry::new(
                    path,
                    self.level,
                    VfsDirectoryOrFile::File(name),
                ));
            } else {
                self.stack.pop();
                self.path_stack.pop();
                self.level -= 1;
                return self.next();
            }
        }

        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VfsWalkerEntry<'a> {
    pub path: VfsPath,
    pub level: usize,
    pub entry: VfsDirectoryOrFile<'a>,
}

impl<'a> VfsWalkerEntry<'a> {
    pub fn new(path: VfsPath, level: usize, entry: VfsDirectoryOrFile<'a>) -> Self {
        Self { path, level, entry }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VfsDirectoryOrFile<'a> {
    Directory(&'a str),
    File(&'a str),
}

#[derive(Debug, Clone)]
struct WalkerLevel<'a> {
    directories: btree_map::Iter<'a, String, VfsDirectory>,
    files: btree_map::Iter<'a, String, usize>,
}
