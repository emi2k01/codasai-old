use codasai_types::{VfsDirectoryOrFile, VfsPath, VfsSnapshot, VfsWalkerEntry};
use yew::{html, Callback, Component, ComponentLink, Properties};

#[derive(Debug, Clone, PartialEq)]
pub enum ExplorerMessage {
    OpenFile(VfsPath),
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct FileExplorerProperties {
    pub snapshot: VfsSnapshot,
    pub on_open_file: Callback<VfsPath>,
}

pub struct FileExplorer {
    props: FileExplorerProperties,
    link: ComponentLink<Self>,
}

impl Component for FileExplorer {
    type Message = ExplorerMessage;
    type Properties = FileExplorerProperties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            ExplorerMessage::OpenFile(path) => {
                self.props.on_open_file.emit(path);
            },
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> yew::Html {
        let walker = self.props.snapshot.walk();
        html! {
            <>
            <div class="file-explorer">
                <h3>{ "Explorer" }</h3>
                {for walker.map(|entry| {
                    self.view_entry(&entry)
                })}
            </div>
            </>
        }
    }
}

impl FileExplorer {
    fn view_entry(&self, entry: &VfsWalkerEntry) -> yew::Html {
        let (entry_class, name, data_file) = match entry.entry {
            VfsDirectoryOrFile::Directory(name) => ("directory", name, None),
            VfsDirectoryOrFile::File(name) => ("file", name, Some(entry.path.clone())),
        };

        let entry_class = yew::classes!(entry_class, "entry");
        let style = format!(
            "padding-left: calc({} * var(--entry-level-padding))",
            entry.level
        );

        let on_open_file = self
            .link
            .batch_callback(move |_| data_file.clone().map(ExplorerMessage::OpenFile));

        html! {
            <div class=entry_class onclick=on_open_file>
                <div class="label" style=style>
                <i class="icon fas fa-angry"></i>
                <span>{ name }</span>
                </div>
                </div>
        }
    }
}
