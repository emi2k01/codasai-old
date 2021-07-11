use std::path::PathBuf;

use include_dir::{include_dir, Dir};
use rocket::response::content::{Html, Json};
use rocket::{State, get, routes};

use crate::file::EmbeddedFile;
pub use crate::state::SharedState;

mod file;
mod state;

const PUBLIC_DIR: Dir = include_dir!("./dist/");

pub type GuideJson = String;

#[derive(Debug, Clone, PartialEq)]
pub struct Server {
    state: SharedState<GuideJson>,
}

impl Server {
    pub fn new(state: SharedState<GuideJson>) -> Self {
        Self { state }
    }

    pub fn launch(self) {
        rocket::async_main(async move {
            rocket::build()
                .mount("/", routes![index, public, get_guide])
                .manage(self.state)
                .ignite()
                .await?
                .launch()
                .await
        })
        .unwrap()
    }
}

#[get("/")]
fn index() -> Html<&'static str> {
    let index_html = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/dist/index.html"));

    Html(index_html)
}

#[get("/guide")]
fn get_guide(guide: &State<SharedState<GuideJson>>) -> Json<String> {
    Json(guide.inner().get())
}

#[get("/public/<path..>")]
fn public(path: PathBuf) -> Option<EmbeddedFile> {
    if let Some(file) = PUBLIC_DIR.get_file(&path) {
        Some(EmbeddedFile(file.path(), file.contents()))
    } else {
        None
    }
}
