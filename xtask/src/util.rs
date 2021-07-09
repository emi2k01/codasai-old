pub mod path {
    use std::path::{Path, PathBuf};

    pub fn project_root() -> &'static Path {
        static MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

        Path::new(MANIFEST_DIR).parent().unwrap()
    }

    pub fn codasai_server_dir() -> PathBuf {
        project_root().join("codasai-server")
    }

    pub fn codasai_web_dir() -> PathBuf {
        project_root().join("codasai-web")
    }
}

pub mod print {
    use colored::{Color, Colorize};

    pub fn task(info: impl AsRef<str>) {
        let info = format!("{}", info.as_ref());
        print_info(&info, Color::Green);
    }

    pub fn step(info: impl AsRef<str>) {
        let info = format!("   {}", info.as_ref());
        print_info(&info, Color::Blue);
    }

    fn print_info(info: &str, title_color: Color) {
        let mut components = info.split_inclusive(":");
        let title = components.next().unwrap().color(title_color).bold();
        let message = components.next().unwrap_or("");

        println!("{}{}", title, message);
    }
}
