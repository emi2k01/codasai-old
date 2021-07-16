use dissimilar::Chunk;
use yew::{classes, html, Component, Properties};

use crate::highlighted_chunk::HighlightedChunk;

mod highlight;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct EditorProperties {
    pub name: String,
    pub old_content: Option<String>,
    pub new_content: String,
    pub highlighted_chunks: Vec<HighlightedChunk>,
}

pub enum EditorMessage {
    ToggleView,
}

pub struct Editor {
    props: EditorProperties,
    old_content: String,
    new_content: String,
    showing_old: bool,
    link: yew::ComponentLink<Self>,
}

impl Component for Editor {
    type Message = EditorMessage;
    type Properties = EditorProperties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        let (old_content, new_content) = diffed_content_from_properties(&props);

        Self {
            props,
            old_content,
            new_content,
            showing_old: false,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            EditorMessage::ToggleView => {
                if self.props.old_content.is_some() {
                    self.showing_old = !self.showing_old;
                }
            },
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        if self.props != props {
            self.showing_old = false;

            let (old_content, new_content) = diffed_content_from_properties(&props);
            self.old_content = old_content;
            self.new_content = new_content;

            self.props = props;

            true
        } else {
            false
        }
    }

    fn view(&self) -> yew::Html {
        let toggle_view = self.link.callback(|_| EditorMessage::ToggleView);

        let (content, class) = if self.showing_old {
            (&self.old_content, "showing-old")
        } else {
            (&self.new_content, "showing-new")
        };

        let icon_hidden = self.props.old_content.is_none();

        html! {
            <div class="editor">
                <div class=classes!("file-name", class) onclick=toggle_view>
                    <span>{ &self.props.name }</span>
                    <i hidden=icon_hidden class="diff-icon diff-new-icon fas fa-plus-square"></i>
                    <i hidden=icon_hidden class="diff-icon diff-old-icon fas fa-minus-square"></i>
                </div>
                <div class="inner">
                    { for content.split('\n').enumerate().map(|(n, l)| self.view_line(l, n+1)) }
                </div>
            </div>
        }
    }
}

impl Editor {
    fn view_line(&self, line: &str, number: usize) -> yew::Html {
        let div = yew::utils::document().create_element("code").unwrap();
        div.set_class_name("content");
        div.set_inner_html(line);

        let line = yew::Html::VRef(div.into());
        let mut highlight_class = None;

        for chunk in &self.props.highlighted_chunks {
            if chunk.line_range.contains(&number) {
                highlight_class = Some(chunk.class.clone());
            }
        }

        html! {
            <div class="line">
                <div class=classes!("number", highlight_class)>
                    { number }
                </div>
                { line }
            </div>
        }
    }
}

fn diffed_content_from_properties(props: &EditorProperties) -> (String, String) {
    let mut old_diffed_content = String::new();
    let mut new_diffed_content = String::new();

    if let Some(old_content) = props.old_content.as_ref() {
        for chunk in dissimilar::diff(&old_content, &props.new_content) {
            match chunk {
                Chunk::Equal(c) => {
                    old_diffed_content.push_str(c);
                    new_diffed_content.push_str(c);
                },
                Chunk::Insert(c) => {
                    let lines = c
                        .split('\n')
                        .map(|l| format!("<span class=\"inserted\">{}</span>", l))
                        .collect::<Vec<_>>()
                        .join("\n");
                    new_diffed_content.push_str(&lines);
                },
                Chunk::Delete(c) => {
                    let lines = c
                        .split('\n')
                        .map(|l| format!("<span class=\"deleted\">{}</span>", l))
                        .collect::<Vec<_>>()
                        .join("\n");
                    old_diffed_content.push_str(&lines);
                },
            }
        }
    } else {
        new_diffed_content = props.new_content.clone();
    }

    (old_diffed_content, new_diffed_content)
}
