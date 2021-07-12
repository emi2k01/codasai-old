use std::fs::{DirBuilder, File, OpenOptions};
use std::path::Path;

use anyhow::{Context, Result};
use codasai_types::{Guide, Vfs, VfsPath, VfsSnapshot};
use git2::{Delta, DiffOptions, Oid, Repository, Tree};

use crate::config::{GuideConfig, PageConfig};
use crate::opts::BuildOpts;
use self::markdown::markdown_to_html;

mod highlight;
mod markdown;

pub fn build(opts: &BuildOpts) -> Result<()> {
    let guide = guide_from_git(&opts.guide)?;
    write_out_file(&guide, &opts.guide.join(".codasai/out/guide.json"))
}

fn write_out_file(guide: &Guide, path: &Path) -> Result<()> {
    create_out_dir(&path.parent().unwrap())?;
    let out_file = create_out_file(path)?;

    if cfg!(debug_assertions) {
        serde_json::to_writer_pretty(out_file, guide)
            .with_context(|| format!("failed to write to file {:?}", path))
    } else {
        serde_json::to_writer(out_file, guide)
            .with_context(|| format!("failed to write to file {:?}", path))
    }
}

fn create_out_dir(path: &Path) -> Result<()> {
    DirBuilder::new()
        .recursive(true)
        .create(path)
        .with_context(|| format!("failed to create directory {:?}", path))
}

fn create_out_file(path: &Path) -> Result<File> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .with_context(|| format!("failed to create file {:?}", path))
}

fn guide_from_git(repo_path: impl AsRef<Path>) -> Result<Guide> {
    let repo_path = repo_path.as_ref();

    let config = GuideConfig::from_file(repo_path.join(".codasai/guide.toml"))?;

    let mut guide = Guide {
        name: config.title.clone(),
        vfs: Vfs::new(),
    };

    let repo = git2::Repository::open(repo_path)
        .with_context(|| format!("failed to open git repository at {:?}", repo_path))?;

    let mut rev_walker = repo.revwalk()?;
    rev_walker.set_sorting(git2::Sort::REVERSE)?;
    rev_walker.push_head().context("you have not saved any page")?;
    let revs = rev_walker.filter_map(Result::ok).collect::<Vec<_>>();

    // Populate first snapshot with first rev
    if let Some(first_rev) = revs.first() {
        add_snapshot_from_rev(&config, &mut guide.vfs, &repo, *first_rev)?;

        let rev_config = get_page_config(&repo, *first_rev)?;
        let page = get_page_in_rev(&rev_config, &repo, *first_rev)?;

        let page_html = markdown_to_html(&page);
        guide.vfs.snapshots.last_mut().unwrap().set_page(page_html);
    }

    // Populate snapshots by applying diff between trees
    for revs_pair in revs.windows(2) {
        let new_snapshot = guide.vfs.add_snapshot();
        let new_snapshot_copy = new_snapshot.clone();

        let old_rev = revs_pair[0];
        let new_rev = revs_pair[1];

        add_snapshot_from_rev_pair(&config, new_snapshot, &repo, old_rev, new_rev)?;

        let rev_config = get_page_config(&repo, new_rev)?;
        let page = get_page_in_rev(&rev_config, &repo, new_rev)?;

        let page_html = markdown_to_html(&page);
        new_snapshot.set_page(page_html);

        // If the snapshot didn't change, then don't remove it
        if *new_snapshot == new_snapshot_copy {
            guide.vfs.snapshots.pop();
        }
    }

    Ok(guide)
}

/// Populates a snapshot by replicating the tree from `rev`. This is intended to
/// be used for the first snapshot only. If used with other snapshots than the
/// first, the old contents will not be removed.
fn add_snapshot_from_rev(
    config: &GuideConfig, vfs: &mut Vfs, repo: &Repository, rev: Oid,
) -> Result<()> {
    let tree = repo.find_commit(rev)?.tree()?;

    let snapshot = vfs.add_snapshot();
    tree.walk(git2::TreeWalkMode::PreOrder, |parent, entry| {
        let file_path = Path::new(parent).join(entry.name().expect("expected a UTF-8 valid name"));

        if file_path.starts_with(".codasai/") || file_path.starts_with(&config.pages_path) {
            return git2::TreeWalkResult::Skip;
        }

        let file_path_vfs = VfsPath::new(&file_path).unwrap();
        let object = BlobOrDirectory::from_git_file(repo, &tree, &file_path).unwrap();

        match object {
            BlobOrDirectory::Blob(content) => snapshot.create_file(&file_path_vfs, content),
            BlobOrDirectory::Directory => snapshot.create_directory(&file_path_vfs),
        }

        git2::TreeWalkResult::Ok
    })?;

    Ok(())
}

/// Adds a snapshot that matches the `new_rev` tree by applying the diff between
/// `old_rev` and `new_rev`
fn add_snapshot_from_rev_pair(
    config: &GuideConfig, snapshot: &mut VfsSnapshot, repo: &Repository, old_rev: Oid, new_rev: Oid,
) -> Result<()> {
    let old_tree = repo.find_commit(old_rev)?.tree()?;
    let new_tree = repo.find_commit(new_rev)?.tree()?;

    let mut diff_opts = DiffOptions::new();
    diff_opts.minimal(true).patience(true);

    let diff = repo.diff_tree_to_tree(Some(&old_tree), Some(&new_tree), Some(&mut diff_opts))?;

    for delta in diff.deltas() {
        let old_file_path = delta
            .old_file()
            .path()
            .expect("`DiffFile` does not have a path");

        let new_file_path = delta
            .new_file()
            .path()
            .expect("`DiffFile` does not have a path");

        // Skip files that are part of .codasai/ or _pages/
        if (old_file_path.starts_with(".codasai/") || old_file_path.starts_with(&config.pages_path))
            && (new_file_path.starts_with(".codasai/")
                || new_file_path.starts_with(&config.pages_path))
        {
            continue;
        }

        let old_file_path_vfs = VfsPath::new(&old_file_path)?;
        let new_file_path_vfs = VfsPath::new(&new_file_path)?;

        match delta.status() {
            Delta::Added => {
                let new_object = BlobOrDirectory::from_git_file(repo, &new_tree, new_file_path)?;
                match new_object {
                    BlobOrDirectory::Blob(content) => {
                        snapshot.create_file(&new_file_path_vfs, content)
                    },
                    BlobOrDirectory::Directory => snapshot.create_directory(&new_file_path_vfs),
                }
            },
            Delta::Deleted => {
                let old_object = BlobOrDirectory::from_git_file(repo, &old_tree, old_file_path)?;
                match old_object {
                    BlobOrDirectory::Blob(_) => snapshot.delete_file(&old_file_path_vfs),
                    BlobOrDirectory::Directory => snapshot.delete_directory(&old_file_path_vfs),
                }
            },
            Delta::Renamed => {
                let old_object = BlobOrDirectory::from_git_file(repo, &old_tree, old_file_path)?;
                let new_object = BlobOrDirectory::from_git_file(repo, &new_tree, new_file_path)?;
                match (old_object, new_object) {
                    (BlobOrDirectory::Blob(_), BlobOrDirectory::Blob(_)) => {
                        snapshot.rename_file(&old_file_path_vfs, &new_file_path_vfs)
                    },
                    (BlobOrDirectory::Directory, BlobOrDirectory::Directory) => {
                        snapshot.rename_directory(&old_file_path_vfs, &new_file_path_vfs)
                    },
                    _ => unreachable!("you shouldn't be able to rename a file to another type :/"),
                }
            },
            Delta::Modified => {
                let old_object = BlobOrDirectory::from_git_file(repo, &old_tree, old_file_path)?;
                let new_object = BlobOrDirectory::from_git_file(repo, &new_tree, new_file_path)?;
                match (old_object, new_object) {
                    (BlobOrDirectory::Blob(_), BlobOrDirectory::Blob(new_content)) => {
                        snapshot.write_file(&new_file_path_vfs, new_content);
                    },
                    _ => {
                        unreachable!(
                            "you shouldn't be able to modify the contents of types that are not \
                             blobs :/"
                        )
                    },
                }
            },
            _ => {},
        }
    }

    Ok(())
}

fn get_page_config(repo: &Repository, rev: Oid) -> Result<PageConfig> {
    let tree = repo.find_commit(rev)?.tree()?;

    let rev_entry = tree.get_path(Path::new(".codasai/rev.toml"))?;
    let rev_entry_object = rev_entry.to_object(&repo)?;
    let rev_entry_bytes = rev_entry_object.as_blob().unwrap().content();
    let rev_entry_string = String::from_utf8(rev_entry_bytes.to_vec())?;
    let rev_config = PageConfig::from_str(&rev_entry_string)?;

    Ok(rev_config)
}

fn get_page_in_rev(rev_config: &PageConfig, repo: &Repository, rev: Oid) -> Result<String> {
    let tree = repo.find_commit(rev)?.tree()?;
    let page_entry = tree.get_path(&rev_config.page_path)?;
    let page_object = page_entry.to_object(repo)?;
    let page_blob = page_object.as_blob().unwrap();
    let page_content = String::from_utf8(page_blob.content().to_vec())?;

    Ok(page_content)
}

enum BlobOrDirectory {
    Blob(String),
    Directory,
}

impl BlobOrDirectory {
    fn from_git_file(repo: &Repository, tree: &Tree, file_path: &Path) -> Result<Self> {
        let object = tree.get_path(file_path)?.to_object(repo)?;
        let is_dir = object.as_tree().is_some();

        if is_dir {
            Ok(Self::Directory)
        } else {
            let object_bytes = object
                .as_blob()
                .expect("expected `object` to be a blob")
                .content();
            let object_content =
                String::from_utf8(object_bytes.to_vec()).unwrap_or(String::from("binary data"));

            Ok(Self::Blob(object_content))
        }
    }
}
