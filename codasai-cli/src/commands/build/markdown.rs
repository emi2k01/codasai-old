use md::CowStr;
use pulldown_cmark as md;

pub fn markdown_to_html(markdown: &str) -> String {
    let mut opts = pulldown_cmark::Options::empty();
    opts.insert(md::Options::ENABLE_FOOTNOTES);
    opts.insert(md::Options::ENABLE_SMART_PUNCTUATION);
    opts.insert(md::Options::ENABLE_STRIKETHROUGH);
    opts.insert(md::Options::ENABLE_TABLES);
    opts.insert(md::Options::ENABLE_TASKLISTS);

    let mut last_lang = None;

    let parser = md::Parser::new_ext(&markdown, opts).map(move |e| {
        match e {
            md::Event::Start(md::Tag::CodeBlock(md::CodeBlockKind::Fenced(lang))) => {
                last_lang = Some(lang.to_string());
                md::Event::Start(md::Tag::CodeBlock(md::CodeBlockKind::Fenced(lang))) 
            },
            md::Event::Text(code) => {
                if let Some(lang) = &last_lang {
                    let mut escaped_code = String::new();
                    // It seems to only fail when the writer fails????
                    md::escape::escape_html(&mut escaped_code, &code.to_string().trim()).unwrap();
                    let highlighted = highlight_codeblock(&escaped_code, lang);

                    last_lang = None;

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
        .add_tag_attributes("span", &["style"])
        .clean(&page_html)
        .to_string();

    clean_page_html
}

fn highlight_codeblock(code: &str, lang: &str) -> String {
    let mut codeblock = String::new();
    codeblock.push_str(&super::highlight::highlight_to_html(code, lang));
    codeblock
}