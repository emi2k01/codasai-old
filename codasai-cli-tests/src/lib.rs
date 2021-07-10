use std::cmp::Ordering;
use std::ffi::OsStr;
use std::iter::repeat;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use mktemp::Temp;

const CODASAI_CLI: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/debug/codasai-cli");
const TMP_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/debug/tmp");

pub struct ProjectOutput<'a> {
    stdout: String,
    stderr: String,
    tree: String,
    dir: &'a Temp,
}

impl<'a> ProjectOutput<'a> {
    pub fn stdout(&self) -> &str {
        &self.stdout
    }

    pub fn stderr(&self) -> &str {
        &self.stderr
    }

    pub fn tree(&self) -> &str {
        &self.tree
    }

    pub fn contents(&self, path: impl AsRef<Path>) -> String {
        let absolute_path = self.dir.to_path_buf().join(path);

        std::fs::read_to_string(&absolute_path).expect("a valid path to a file")
    }
}

pub struct Project {
    cwd: PathBuf,
    tmp_dir: Temp,
}

impl Project {
    pub fn new() -> Self {
        mktmp();

        let project_dir = Temp::new_dir_in(TMP_DIR).expect("create tmp dir correctly");
        println!("{:?}", project_dir.to_path_buf());
        Project {
            cwd: project_dir.to_path_buf(),
            tmp_dir: project_dir,
        }
    }

    pub fn run(&self, cmd: &str, args: &[&str]) -> ProjectOutput {
        let process = Command::new(CODASAI_CLI)
            .arg(cmd)
            .args(args)
            .current_dir(&self.cwd)
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn codasai-cli process");

        let output = process
            .wait_with_output()
            .expect("wait codasai-cli process with output");

        ProjectOutput {
            stdout: String::from_utf8(output.stdout).expect("valid utf8 stdout"),
            stderr: String::from_utf8(output.stderr).expect("valid utf8 stderr"),
            tree: serialize_tree(&self.tmp_dir),
            dir: &self.tmp_dir,
        }
    }
}

fn mktmp() {
    let tmp_dir = PathBuf::from(TMP_DIR);
    let mkres = std::fs::DirBuilder::new().recursive(true).create(tmp_dir);

    if !matches!(&mkres, Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists) {
        mkres.unwrap();
    }
}

fn serialize_tree(dir: &Temp) -> String {
    let dir_path = dir.to_path_buf();

    let mut walk_builder = ignore::WalkBuilder::new(dir.as_path());

    walk_builder
        .standard_filters(false)
        .sort_by_file_path(sort_tree_entries)
        .filter_entry(move |e| {
            e.path().strip_prefix(&dir_path).unwrap().file_name() != Some(OsStr::new(".git"))
        })
        .add_ignore(dir.as_path().join(".git/"))
        .unwrap();

    let walker = walk_builder
        .build()
        .skip(1) // Skip the root directory. Only the contents matter.
        .filter_map(Result::ok)
        .map(|e| {
            let (depth, mut file_name) = (
                e.depth(),
                e.path().file_name().unwrap().to_str().unwrap().to_owned(),
            );

            if e.path().is_dir() {
                file_name.push('/');
            }

            (depth, file_name)
        });

    let mut tree = String::new();
    for (depth, file_name) in walker {
        tree.extend(repeat(' ').take((depth - 1) * 4));
        tree.push_str(&file_name);
        tree.push('\n');
    }

    let _last_newline = tree.pop();

    tree
}

/// Sorts the entries by the following rules
///
/// - A directory goes before a file
/// - Entries of the same type are sorted lexicographically
fn sort_tree_entries(l: &Path, r: &Path) -> Ordering {
    if l.is_dir() && r.is_file() {
        Ordering::Less
    } else if r.is_file() && l.is_dir() {
        Ordering::Greater
    } else {
        l.cmp(r)
    }
}
