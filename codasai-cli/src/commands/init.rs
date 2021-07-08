use std::fs;
use std::io::{ErrorKind, Write};
use std::path::Path;

use anyhow::{ensure, Context, Result};
use indoc::writedoc;

use crate::opts::InitOpts;

pub fn init(opts: &InitOpts) -> Result<()> {
    ensure!(
        !dir_exists(".codasai")?,
        ".codasai directory already exists"
    );

    let title = &opts.title;

    if !dir_exists(".git")? {
        init_git_repo()?;
    }
    create_dotcodasai_dir(&title)?;
    create_pages_dir()?;

    Ok(())
}

fn create_dotcodasai_dir(title: &str) -> Result<()> {
    fs::create_dir("./.codasai")?;

    // .codasai/guide.toml
    let mut guide_toml = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("./.codasai/guide.toml")?;

    writedoc!(
        guide_toml,
        r#"
            title = "{}"
        "#,
        title
    )?;

    // .codasai/rev.toml
    let mut guide_toml = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("./.codasai/rev.toml")?;

    writedoc!(
        guide_toml,
        r#"
            page_path = "_pages/introduction.md"
        "#,
    )?;

    Ok(())
}

fn create_pages_dir() -> Result<()> {
    let pages_path = Path::new("./_pages");
    fs::create_dir(pages_path)
        .with_context(|| format!("failed to create directory at {:?}", pages_path))?;

    let introduction_md_path = Path::new("./_pages/introduction.md");
    let mut introduction_md = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(introduction_md_path)
        .with_context(|| format!("failed to create file at {:?}", introduction_md_path))?;

    writedoc!(
        introduction_md,
        "
            # Introduction

            **You are about to read an amazing guide!**
        "
    )?;

    Ok(())
}

fn dir_exists(dir: impl AsRef<Path>) -> Result<bool> {
    let dir = dir.as_ref();

    if let Err(e) = fs::read_dir(dir) {
        if e.kind() == ErrorKind::NotFound {
            return Ok(false);
        } else {
            return Err(e).with_context(|| format!("failed to read {:?} directory", dir));
        }
    } else {
        return Ok(true);
    }
}

fn init_git_repo() -> Result<()> {
    git2::Repository::init(".")?;
    Ok(())
}
