use codasai_cli_tests::Project;

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
