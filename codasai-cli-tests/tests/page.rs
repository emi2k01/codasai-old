use codasai_cli_tests::{Project, WildStr};

#[test]
fn page_save_simple() {
    let project = Project::new();
    project.run("init", &["Simple guide"]);
    project.run("page", &["new", "Introduction"]);
    let output = project.run("page", &["save", "-m", "Page: Introduction"]);

    assert_eq!(
        WildStr::from(output.stdout()),
        "[..] Page: Introduction
 3 files changed, 3 insertions(+)
 create mode 100644 .codasai/guide.toml
 create mode 100644 .codasai/rev.toml
 create mode 100644 _pages/introduction.md
"
    );

    k9::snapshot!(output.stderr(), "");

    k9::snapshot!(
        output.tree(),
        "
.codasai/
    guide.toml
    rev.toml
_pages/
    introduction.md
"
    );
}

#[test]
fn page_new_simple() {
    let project = Project::new();
    project.run("init", &["Simple guide"]);

    let output = project.run("page", &["new", "Setting up the environment"]);

    k9::snapshot!(output.stdout(), "");

    k9::snapshot!(output.stderr(), "");

    k9::snapshot!(
        output.tree(),
        "
.codasai/
    guide.toml
    rev.toml
_pages/
    setting-up-the-environment.md
"
    );

    k9::snapshot!(
        output.contents("_pages/setting-up-the-environment.md"),
        "
# Setting up the environment

"
    );

    k9::snapshot!(
        output.contents(".codasai/rev.toml"),
        r#"
page_path = "_pages/setting-up-the-environment.md"

"#
    );
}
