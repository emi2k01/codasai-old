use std::collections::HashSet;

use md::CowStr;
use pulldown_cmark as md;

use syntect::{html::{ClassStyle, ClassedHTMLGenerator}, parsing::{ParseState, ScopeStackOp, SyntaxSet}, util::LinesWithEndings};

thread_local! {
    static SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();
}

pub fn markdown_to_html(markdown: &str) -> String {
    let mut last_lang = None;
    let mut classes = HashSet::new();
    let classes_ref = &mut classes;

    let parser = md::Parser::new_ext(&markdown, md::Options::all()).map(move |e| {
        match e {
            md::Event::Start(md::Tag::CodeBlock(md::CodeBlockKind::Fenced(ref lang))) => {
                last_lang = Some(lang.to_string());
                e
            },
            md::Event::Text(code) => {
                // Convert to an html event if we are in a fenced code block
                if let Some(lang) = last_lang.take() {
                    let highlighted = highlight_codeblock(&code, &lang, classes_ref);

                    md::Event::Html(CowStr::from(highlighted))
                } else {
                    md::Event::Text(code)
                }
            },
            _ => e
        }
    });

    let mut page_html = String::new();
    md::html::push_html(&mut page_html, parser);

    let clean_page_html = ammonia::Builder::new()
        .add_generic_attributes(&["data-rel"])
        .add_allowed_classes("span", &classes)
        .clean(&page_html)
        .to_string();

    clean_page_html
}

/// Returns the highlighted string as a html code tag and the classes it used.
fn highlight_codeblock(code: &str, lang: &str, classes: &mut HashSet<String>) -> String {
    let mut codeblock = String::new();
    codeblock.push_str(&highlight_to_html(code, lang, classes));
    codeblock
}

/// Returns the highlighted string as a series of html span tags and the classes it used.
///
/// It escapes the code.
pub fn highlight_to_html(code: &str, lang: &str, classes: &mut HashSet<String>) -> String {
    const CLASS_PREFIX: &str = "cbsh-";

    SYNTAX_SET.with(|syntax_set| {
        let syntax = syntax_set.find_syntax_by_token(lang).unwrap_or_else(|| syntax_set.find_syntax_by_token("txt").unwrap());

        let mut gen = ClassedHTMLGenerator::new_with_class_style(syntax, syntax_set, ClassStyle::SpacedPrefixed { prefix: CLASS_PREFIX });
        let mut parser = ParseState::new(&syntax);

        for line in LinesWithEndings::from(code) {
            // We need to add a class for each atom
            let scopes = parser.parse_line(line, syntax_set).into_iter().map(|(_, op)| op).filter_map(|op| match op { ScopeStackOp::Push(scope) => Some(scope), _ => None });
            for scope in scopes {
                for atom in scope.build_string().split('.') {
                    classes.insert(format!("{}{}", CLASS_PREFIX, atom));
                }
            }

            gen.parse_html_for_line_which_includes_newline(line);
        }

        gen.finalize()
    })
}
