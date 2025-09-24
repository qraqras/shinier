use crate::{
    doc::{Doc, Docs, sequence, text},
    softline,
};

pub fn bracketed_layout(doc: &Doc, open_delimiter: &str, close_delimiter: &str) -> Doc {
    let mut layout = Vec::with_capacity(3);
    layout.push(text(open_delimiter));
    layout.push(doc.clone());
    layout.push(text(close_delimiter));
    sequence(layout)
}

pub fn list_layout(
    docs: &Docs,
    open_delimiter: &str,
    close_delimiter: &str,
    separator: &str,
) -> Doc {
    let mut separated = Vec::with_capacity((docs.len() * 2) - 1);
    for (i, doc) in docs.iter().enumerate() {
        if i > 0 {
            separated.push(text(separator));
            separated.push(softline());
        }
        separated.push(doc.clone());
    }
    bracketed_layout(&sequence(separated), open_delimiter, close_delimiter)
}
