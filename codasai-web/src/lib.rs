use codasai_types::Guide;
use wasm_bindgen::prelude::*;
use web_sys::Element;

mod app;
mod components;
mod highlighted_chunk;

#[wasm_bindgen]
pub fn start(guide: &str, el: Element) {
    let guide = Guide::from_json(guide).expect("a valid a guide encoded as json");

    yew::App::<app::App>::new().mount_with_props(el, app::AppProps { guide });
}
