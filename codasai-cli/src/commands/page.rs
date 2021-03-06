use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context, Result};
use git2::Repository;
use indoc::writedoc;
use slug::slugify;
use toml_edit;

use crate::config::PageConfig;
use crate::opts::{PageNewOpts, PageOpts, PageSaveOpts, PageSubcmd};
use crate::util::{self, path};

pub fn page(opts: &PageOpts) -> Result<()> {
    match opts.subcmd {
        PageSubcmd::New(ref opts) => new(opts),
        PageSubcmd::Save(ref opts) => save(opts),
    }
}

pub fn new(opts: &PageNewOpts) -> Result<()> {
    let repo_path = PathBuf::from(".").canonicalize()?;

    let repo = Repository::open(".")
        .with_context(|| format!("failed to open repository at {:?}", repo_path))?;

    if let Some(unsaved_page) = find_unsaved_page(&repo)? {
        bail!(
            "There is an unsaved page at {:?}. Discard it or save it before retrying.",
            unsaved_page
        );
    }

    let title = &opts.title;

    let new_page_path = Path::new("_pages/").join(format!("{}.md", slugify(&title)));

    let mut new_page = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&new_page_path)
        .with_context(|| format!("failed to create page at {:?}", new_page_path))?;

    writedoc!(
        new_page,
        "
        # {}
        ",
        title
    )
    .with_context(|| format!("failed to write to file at {:?}", new_page_path))?;

    let rev_toml_path = path::dotcodasai()?.join("rev.toml");
    let mut rev_toml = fs::read_to_string(&rev_toml_path)?.parse::<toml_edit::Document>()?;
    rev_toml["page_path"] = toml_edit::value(new_page_path.to_string_lossy().into_owned());
    fs::write(&rev_toml_path, rev_toml.to_string().as_bytes())?;

    Ok(())
}

pub fn save(opts: &PageSaveOpts) -> Result<()> {
    let rev_toml_path = util::path::dotcodasai()?
        .join("rev.toml")
        .canonicalize()
        .context("failed to canonicalize path to rev.toml")?;
    let rev_toml_string = fs::read_to_string(&rev_toml_path).context("failed to read rev.toml")?;
    let rev_toml: PageConfig = toml::from_str(&rev_toml_string).context("failed to parse rev.toml")?;

    if rev_toml.page_path.to_str() == Some("") {
        bail!("you have not created a page yet");
    }

    if !rev_toml.page_path.exists() {
        bail!("page `{}` does not exist", rev_toml.page_path.display());
    }

    Command::new("git").args(&["add", "-A"]).output()?;

    let mut commit_command = Command::new("git");
    commit_command.arg("commit");

    if let Some(ref message) = opts.message {
        commit_command.args(&["-m", message]);
    }

    commit_command.spawn()?.wait_with_output()?;

    Ok(())
}

fn find_unsaved_page(repo: &Repository) -> Result<Option<PathBuf>> {
    let statuses = repo
        .statuses(None)
        .context("failed to get status of repository")?;

    for status in statuses.iter() {
        let path = status.path().context("unexpected non utf-8 file path")?;
        let path = PathBuf::from(path);
        if path.starts_with("_pages/") {
            return Ok(Some(path));
        }
    }

    Ok(None)
}
