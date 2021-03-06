use codasai_cli_tests::Project;

#[test]
fn codasai_cli_init_simple() {
    let project = Project::new();
    let output = project.run("init", &["Simple Guide"]);

    k9::snapshot!(output.stdout(), "");

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

    k9::snapshot!(
        output.contents(".codasai/guide.toml"),
        r#"
title = "Simple Guide"

"#
    );
}

#[test]
fn codasai_cli_init_twice() {
    let project = Project::new();
    let _output1 = project.run("init", &["Simple Guide"]);
    let output2 = project.run("init", &["Simple Guide"]);

    k9::snapshot!(output2.stdout(), "");

    k9::snapshot!(
        output2.stderr(),
        "
Error: .codasai directory already exists

"
    );
}
