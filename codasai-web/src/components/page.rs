use codasai_types::VfsPath;
use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{Element, EventTarget};
use yew::{Callback, Component, ComponentLink, Html, Properties};

use crate::highlighted_chunk::HighlightedChunk;

#[derive(Debug, Clone)]
pub enum PageMessage {
    OpenFile(VfsPath),
}

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct PageProperties {
    pub content: String,
    pub on_open_file: Callback<VfsPath>,
    pub on_chunk_rels: Callback<Vec<HighlightedChunk>>,
}

pub struct Page {
    props: PageProperties,
    link: ComponentLink<Self>,
    anchor_listeners: Vec<EventListener>,
}

impl Component for Page {
    type Message = PageMessage;
    type Properties = PageProperties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            anchor_listeners: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            PageMessage::OpenFile(file_path) => self.props.on_open_file.emit(file_path),
        };

        false
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let should_render = if self.props.content != props.content {
            self.anchor_listeners.clear();
            true
        } else {
            false
        };

        self.props = props;

        should_render
    }

    fn view(&self) -> yew::Html {
        let div = yew::utils::document().create_element("div").unwrap();
        div.set_class_name("markdown-body page");
        div.set_inner_html(&self.props.content);

        Html::VRef(div.into())
    }

    fn rendered(&mut self, _first_render: bool) {
        self.set_file_links();
        self.set_chunk_rels();
    }

    fn destroy(&mut self) {
        self.anchor_listeners.clear();
    }
}

impl Page {
    fn set_file_links(&mut self) {
        let anchors = yew::utils::document()
            .query_selector_all(".page button[data-file]")
            .unwrap();

        for i in 0..anchors.length() {
            let anchor = anchors.item(i).unwrap();
            let path = anchor
                .dyn_ref::<Element>()
                .unwrap()
                .get_attribute("data-file")
                .unwrap();
            let vfs_path = VfsPath::new(path).unwrap();

            let link = self.link.clone();
            let vfs_path = vfs_path.clone();
            let event_listener =
                EventListener::new(&EventTarget::from(anchor), "click", move |_event| {
                    link.send_message(PageMessage::OpenFile(vfs_path.clone()));
                });
            self.anchor_listeners.push(event_listener);
        }
    }

    fn set_chunk_rels(&mut self) {
        let divs = yew::utils::document()
            .query_selector_all("*[data-rel]")
            .unwrap();

        let mut chunks = vec![];
        for i in 0..divs.length() {
            let div = divs.item(i).unwrap().dyn_into::<Element>().unwrap();
            let rel_attr = div.get_attribute("data-rel").unwrap();

            let class = format!("rel-{}", i);
            let hi_chunk = HighlightedChunk::from_data_rel(&rel_attr, class.clone());

            match hi_chunk {
                Ok(c) => {
                    chunks.push(c);
                    div.class_list().add_1(&class).unwrap();
                },
                Err(_e) => {}, // TODO: warn!(e)
            }
        }
        self.props.on_chunk_rels.emit(chunks);
    }
}
