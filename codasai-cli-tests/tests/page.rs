use codasai_cli_tests::{Project, WildStr};

#[test]
fn page_save_simple() {
    let project = Project::new();
    project.run("init", &["Simple guide"]);
    let output = project.run("page", &["save", "-m", "Page: Introduction"]);

    assert_eq!(
        WildStr::from(output.stdout()),
        "[..] Page: Introduction
 2 files changed, 1 insertion(+)
 create mode 100644 .codasai/guide.toml
 create mode 100644 .codasai/rev.toml
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
