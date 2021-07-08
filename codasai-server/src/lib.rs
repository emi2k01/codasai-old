use std::path::PathBuf;

use include_dir::{include_dir, Dir};
use rocket::response::content::Html;
use rocket::{get, routes};

use crate::file::EmbeddedFile;
pub use crate::state::SharedState;

mod file;
mod state;

const STATIC_DIR: Dir = include_dir!("./static/");

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
                .mount("/", routes![index, public])
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
    let index_html = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/index.html"));

    Html(index_html)
}

#[get("/public/<path..>")]
fn public(path: PathBuf) -> Option<EmbeddedFile> {
    if let Some(file) = STATIC_DIR.get_file(&path) {
        Some(EmbeddedFile(file.path(), file.contents()))
    } else {
        None
    }
}
