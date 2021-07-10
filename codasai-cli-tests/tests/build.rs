use codasai_cli_tests::Project;

#[test]
fn build_simple() {
    let project = Project::new();
    project.run("init", &["Simple guide"]);
    project.run("page", &["new", "Introduction"]);
    project.run("page", &["save", "-m", "\"Page: Introduction\""]);
    let output = project.run("build", &[]);

    k9::snapshot!(output.stdout(), "");

    k9::snapshot!(output.stderr(), "");

    k9::snapshot!(
        output.tree(),
        "
.codasai/
    out/
        guide.json
    guide.toml
    rev.toml
_pages/
    introduction.md
"
    );

    k9::snapshot!(
        output.contents(".codasai/out/guide.json"),
        r#"
{
  "name": "Simple guide",
  "vfs": {
    "files": [],
    "snapshots": [
      {
        "root": {
          "directories": {},
          "files": {}
        },
        "page": "<h1>Introduction</h1>\
"
      }
    ]
  }
}
"#
    );
}
