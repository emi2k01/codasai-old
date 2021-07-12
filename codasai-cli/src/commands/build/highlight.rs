use syntect::{easy::HighlightLines, highlighting::{Style, ThemeSet}, parsing::SyntaxSet, util::LinesWithEndings};

thread_local! {
    static SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();
    static THEME_SET: ThemeSet = ThemeSet::load_defaults();
}

pub fn highlight_to_html(code: &str, lang: &str) -> String {
    SYNTAX_SET.with(|syntax_set| {
        let syntax = syntax_set.find_syntax_by_token(lang).unwrap_or_else(|| syntax_set.find_syntax_by_token("txt").unwrap());
        THEME_SET.with(|theme_set| {
            let theme = theme_set.themes.get("base16-ocean.light").unwrap();

            let mut highlighter = HighlightLines::new(syntax, &theme);

            let mut output = String::new();
            for line in LinesWithEndings::from(code) {
                let ranges: Vec<(Style, &str)> = highlighter.highlight(line, syntax_set);

                for (style, span) in ranges {
                    let fg = style.foreground;
                    output.push_str(&format!("<span style=\"color: rgba({}, {}, {}, {});\">", fg.r, fg.g, fg.b, fg.a));
                    output.push_str(span);
                    output.push_str("</span>");
                }
            }

            output
        })
    })
}
