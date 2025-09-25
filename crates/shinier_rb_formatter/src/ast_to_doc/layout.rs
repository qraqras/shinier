use crate::doc::{Doc, Docs, fill, if_break, indent, line, sequence, softline, text};

pub fn bracketed_layout(doc: &Doc, open_delimiter: &str, close_delimiter: &str) -> Doc {
    let mut layout = Vec::with_capacity(5);
    layout.push(text(open_delimiter));
    layout.push(softline());
    layout.push(indent(doc.clone()));
    layout.push(softline());
    layout.push(text(close_delimiter));
    sequence(layout)
}

pub fn list_layout(
    docs: &Docs,
    open_delimiter: &str,
    close_delimiter: &str,
    separator: &str,
) -> Doc {
    let len = docs.len();
    let mut separated = Vec::with_capacity(((len - 1) * 3) + 2);
    separated.push(softline());
    for (i, doc) in docs.iter().enumerate() {
        separated.push(doc.clone());
        if i < len - 1 {
            separated.push(text(separator));
            separated.push(line());
        }
        separated.push(if_break(text(","), text("")));
    }
    bracketed_layout(&fill(separated), open_delimiter, close_delimiter)
}
