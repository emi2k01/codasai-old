use codasai_types::{Guide, VfsDirectoryOrFile, VfsPath};
use yew::services::keyboard::KeyListenerHandle;
use yew::services::KeyboardService;
use yew::{html, Component, ComponentLink, KeyboardEvent};

use crate::components::{Editor, FileExplorer, Page};
use crate::highlighted_chunk::HighlightedChunk;

pub enum AppMessage {
    PreviousPage,
    NextPage,
    KeyDown(KeyboardEvent),
    OpenFile(VfsPath),
    ChunkRels(Vec<HighlightedChunk>),
}

pub struct App {
    guide: Guide,
    page_number: usize,
    _keyboard_handle: KeyListenerHandle,
    link: ComponentLink<Self>,
    file_path: Option<VfsPath>,
    chunk_rels: Vec<HighlightedChunk>,
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let guide = Guide::from_json(include_str!("/tmp/guide.json")).unwrap();
        Self {
            guide,
            page_number: 0,
            _keyboard_handle: KeyboardService::register_key_down(
                &yew::utils::window(),
                link.callback(|k| AppMessage::KeyDown(k)),
            ),
            link,
            file_path: None,
            chunk_rels: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            AppMessage::PreviousPage => {
                self.page_number = self.page_number.saturating_sub(1);
            },
            AppMessage::NextPage => {
                self.page_number = self.page_number.saturating_add(1);
            },
            AppMessage::KeyDown(e) => {
                match e.key().as_ref() {
                    "ArrowLeft" => {
                        self.page_number = self.page_number.saturating_sub(1);
                    },
                    "ArrowRight" => {
                        self.page_number = self
                            .page_number
                            .saturating_add(1)
                            .min(self.guide.vfs.snapshots.len() - 1);
                    },
                    _ => {},
                }
            },
            AppMessage::OpenFile(path) => {
                self.file_path = Some(path);
            },
            AppMessage::ChunkRels(chunks) => {
                self.chunk_rels = chunks;
            },
        };

        true
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        let snapshot = self.guide.vfs.snapshots[self.page_number].clone();
        let open_file = self.link.callback(|path| AppMessage::OpenFile(path));

        let on_chunk_rels = self.link.callback(|chunks| AppMessage::ChunkRels(chunks));

        html! {
            <div class="app">
                <FileExplorer snapshot=snapshot.clone() on_open_file=open_file.clone() />
                { self.view_editor() }
                <Page content=snapshot.page.clone() on_open_file=open_file on_chunk_rels=on_chunk_rels />

                { self.view_navigation() }
            </div>
        }
    }
}

impl App {
    fn view_editor(&self) -> yew::Html {
        let snapshot = &self.guide.vfs.snapshots[self.page_number];

        let maybe_file_path = self.file_path.clone().or_else(|| {
            snapshot
                .walk()
                .find(|entry| matches!(entry.entry, VfsDirectoryOrFile::File(_)))
                .map(|e| e.path)
        });

        if let Some(file_path) = maybe_file_path {
            if let Some(file_content) = snapshot.read_file(&file_path) {
                let old_file_content = self
                    .guide
                    .vfs
                    .snapshots
                    .get(self.page_number - 1)
                    .and_then(|s| s.read_file(&file_path));

                let highlighted_chunks = self
                    .chunk_rels
                    .iter()
                    .filter(|c| c.file == file_path)
                    .cloned()
                    .collect::<Vec<_>>();

                html! {
                <Editor
                    name=file_path.file_name().to_string()
                    new_content=file_content
                    old_content=old_file_content
                    highlighted_chunks=highlighted_chunks /> }
            } else {
                html! {}
            }
        } else {
            html! {}
        }
    }

    fn view_navigation(&self) -> yew::Html {
        let previous_page = self.link.callback(|_| AppMessage::PreviousPage);
        let next_page = self.link.callback(|_| AppMessage::NextPage);

        html! {
            <div class="navigation">
                { if self.page_number != 0 {
                    html! { <button onclick=previous_page>{ "Previous" }</button> }
                } else {
                    html! { <button class="disabled">{ "Previous" }</button>}
                }}

                { if self.page_number != self.guide.vfs.snapshots.len() - 1 {
                    html! { <button onclick=next_page>{ "Next" }</button> }
                } else {
                    html! { <button class="disabled">{ "Next" }</button> }
                }}
            </div>
        }
    }
}
