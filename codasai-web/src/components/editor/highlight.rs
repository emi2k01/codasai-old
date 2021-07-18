use syntect::{
    highlighting::ThemeSet,
    html::{css_for_theme_with_class_style, ClassStyle},
    parsing::{BasicScopeStackOp, ParseState, Scope, ScopeStack, SyntaxSet, SCOPE_REPO},
    util::LinesWithEndings,
};

use super::escape::Escape;

#[derive(Debug, Clone)]
struct Span<'a> {
    offset: usize,
    text: &'a str,
    class: String,
}

#[derive(Debug, Clone)]
struct HighlightOp {
    offset: usize,
    kind: HighlightOpKind,
}

#[derive(Debug, Clone)]
enum HighlightOpKind {
    Push { class: String },
    Pop,
}

#[derive(Debug, Clone)]
pub struct InsertPoint {
    index: usize,
    push_offset: usize,
    pop_offset: usize,
}

pub fn highlight_and_diff_code(old: &str, new: &str, lang: &str) -> (String, String) {
    let (spans_old, spans_new) = diff(old, new);
    (highlight_diffed_code(old, &spans_old, lang), highlight_diffed_code(new, &spans_new, lang))
}

fn highlight_diffed_code(text: &str, spans: &[Span<'_>], lang: &str) -> String {
    let mut output = String::new();
    let mut offset = 0;
    let mut text_offset = 0;
    let mut text_len = 0;
    for (i, span) in spans.iter().enumerate() {
        text_len += span.text.len();
        if span.text.ends_with("\n") {
            //spans must be modified so that the offsets are based on line start and not text
            // start.
            let mut spans_offseted = Vec::new();
            for span in &spans[offset..i+1] {
                spans_offseted.push(Span {
                    offset: span.offset - text_offset,
                    text: span.text,
                    class: span.class.clone()
                });
            }
            output.push_str(&highlight_diffed_line(&text[text_offset..][..text_len], &spans_offseted, lang));
            offset = i+1;
            text_offset += text_len;
            text_len = 0;
        }
    }

    output
}

fn highlight_diffed_line(line: &str, spans: &[Span<'_>], lang: &str) -> String {
    let stack = highlight_line(line, lang);
    let merged_stack = merge_stack_with_spans(&stack, spans);
    stack_to_html(&merged_stack, line)
}

fn merge_stack_with_spans(stack: &[HighlightOp], spans: &[Span<'_>]) -> Vec<HighlightOp> {
    let mut merged_stack = Vec::new();

    let mut last_index = 0;
    for (insert_points, span) in spans
        .into_iter()
        .map(|span| (find_insert_points(&stack, &span), span))
    {
        for insert_point in insert_points {
            for i in last_index..insert_point.index {
                merged_stack.push(stack[i].clone());
                last_index += 1;
            }
            merged_stack.push(HighlightOp {
                offset: insert_point.push_offset,
                kind: HighlightOpKind::Push {
                    class: span.class.clone(),
                },
            });
            merged_stack.push(HighlightOp {
                offset: insert_point.pop_offset,
                kind: HighlightOpKind::Pop,
            });
        }
    }

    merged_stack
}

fn stack_to_html(stack: &[HighlightOp], line: &str) -> String {
    let mut html = String::new();
    let mut last_char = 0;
    for op in stack {
        if op.offset > last_char {
            html.push_str(&Escape(&line[last_char..op.offset]).to_string());
            last_char = op.offset;
        }

        match op.kind {
            HighlightOpKind::Push { ref class } => {
                html.push_str(&format!("<span class=\"{}\">", class))
            }
            HighlightOpKind::Pop => html.push_str("</span>"),
        }
    }

    html.push_str(&Escape(&line[last_char..]).to_string());

    html
}

fn find_insert_points(stack: &[HighlightOp], span: &Span<'_>) -> Vec<InsertPoint> {
    let mut insert_points = Vec::new();

    let mut span = span.clone();
    for (i, op) in stack.iter().enumerate() {
        // Find a slot in `stack` such that it would stay sorted if `span` was inserted there.
        // That is, find a slot where `stack[i].offset <= span.offset <= stack[i].offset`.
        if op.offset <= span.offset {
            if let Some(peek_op) = stack.get(i + 1) {
                if peek_op.offset >= span.offset {
                    let available_space = peek_op.offset - span.offset;

                    // If it was found, mark this slot as an `InsertPoint`
                    insert_points.push(InsertPoint {
                        index: i + 1,
                        push_offset: span.offset,
                        pop_offset: span.offset + span.text.len().min(available_space),
                    });

                    if available_space < span.text.len() {
                        // If `span` doesn't fit in this slot, then split `span` and
                        // find a slot for the next part.
                        span.text = &span.text[peek_op.offset - span.offset..];
                        span.offset = peek_op.offset
                    } else {
                        // If it fits, we're done.
                        return insert_points;
                    }
                }
            } else {
                // If we're in the final slot, then we put the `InsertPoint` here
                insert_points.push(InsertPoint {
                    index: i + 1,
                    push_offset: span.offset,
                    pop_offset: span.offset + span.text.len(),
                })
            }
        }
    }

    insert_points
}

fn highlight_line<'a>(line: &'a str, lang: &str) -> Vec<HighlightOp> {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let syntax_ref = syntax_set.find_syntax_by_token(lang).unwrap();

    let mut parse_state = ParseState::new(syntax_ref);
    let mut scope_stack = ScopeStack::new();
    let mut output = Vec::new();

    let ops = parse_state.parse_line(line, &syntax_set);

    for (i, op) in ops {
        scope_stack.apply_with_hook(&op, |basic_op, _| match basic_op {
            BasicScopeStackOp::Push(scope) => {
                let mut class = String::new();
                scope_to_classes(&mut class, scope, ClassStyle::Spaced);
                output.push(HighlightOp {
                    offset: i,
                    kind: HighlightOpKind::Push { class },
                });
            }
            BasicScopeStackOp::Pop => output.push(HighlightOp {
                offset: i,
                kind: HighlightOpKind::Pop,
            }),
        });
    }

    output
}

fn scope_to_classes(s: &mut String, scope: Scope, style: ClassStyle) {
    let repo = SCOPE_REPO.lock().unwrap();
    for i in 0..(scope.len()) {
        let atom = scope.atom_at(i as usize);
        let atom_s = repo.atom_str(atom);
        if i != 0 {
            s.push_str(" ")
        }
        match style {
            ClassStyle::SpacedPrefixed { prefix } => {
                s.push_str(&prefix);
            }
            _ => {}
        }
        s.push_str(atom_s);
    }
}

fn diff<'a>(old: &'a str, new: &'a str) -> (Vec<Span<'a>>, Vec<Span<'a>>) {
    use dissimilar::Chunk;

    const DIFF_EQUAL_CLASS: &str = "diff-equal";
    const DIFF_INSERT_CLASS: &str = "diff-insert";
    const DIFF_DELETE_CLASS: &str = "diff-delete";

    let mut spans_old = Vec::new();
    let mut spans_new = Vec::new();

    let mut offset_old = 0;
    let mut offset_new = 0;

    for chunk in dissimilar::diff(old, new) {
        match chunk {
            Chunk::Equal(text) => {
                for line in text.split_inclusive('\n') {
                    spans_old.push(Span {
                        offset: offset_old,
                        text: line,
                        class: DIFF_EQUAL_CLASS.into()
                    });

                    spans_new.push(Span {
                        offset: offset_new,
                        text: line,
                        class: DIFF_EQUAL_CLASS.into()
                    });

                    offset_old += line.len();
                    offset_new += line.len();
                }
            },
            Chunk::Insert(text) => {
                for line in text.split_inclusive('\n') {
                    spans_new.push(Span {
                        offset: offset_new,
                        text: line,
                        class: DIFF_INSERT_CLASS.into()
                    });

                    offset_new += line.len();
                }
            },
            Chunk::Delete(text) => {
                for line in text.split_inclusive('\n') {
                    spans_old.push(Span {
                        offset: offset_old,
                        text: line,
                        class: DIFF_DELETE_CLASS.into()
                    });

                    offset_old += line.len();
                }
            },
        }
    }

    (spans_old, spans_new)
}
