use codasai_cli_tests::Project;

#[test]
fn page_save_simple() {
    let project = Project::new();
    project.run("init", &["Simple guide"]);
    let output = project.run("page", &["save", "-m", "Page: Introduction"]);

    k9::snapshot!(output.stdout(), "");

    k9::snapshot!(output.stderr(), "");

    k9::snapshot!(output.tree(), "");

    // TODO: We need to make sure that when a page is saved, a git commit is
    // created with the given message. We want to avoid exposing git as much
    // as possible. So, to be able to test that a commit is created, a
    // generic facade should be implemented in the CLI tool so that we can
    // output the commit history and at the same time manipulate the output to
    // exclude non-deterministic information.
    //
    // It would look something like:
    //
    // let output = project.run("log", "--deterministic");
    //
    // This would not include things like hash, author, date, etc in the output
    // and only include things like the files changed and the commit
    // message.
}

#[test]
fn page_new_simple() {
    let project = Project::new();
    project.run("init", &["Simple guide"]);
    project.run("page", &["save", "-m", "Page: Introduction"]);

    let output = project.run("page", &["new", "Setting up the environment"]);

    k9::snapshot!(output.stdout(), "");

    k9::snapshot!(output.stderr(), "");

    k9::snapshot!(
        output.tree(),
        "
_pages/
    introduction.md
    setting-up-the-environment.md
.codasai/
    rev.toml
    guide.toml
"
    );

    k9::snapshot!(
        output.contents("_pages/setting-up-the-environment.md"),
        "
# Setting up the environment



"
    );
}
