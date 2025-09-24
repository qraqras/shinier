use crate::doc::{Doc, Docs, sequence, text};

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
    let mut separated = Vec::with_capacity(docs.len().saturating_mul(2));
    for (i, doc) in docs.iter().enumerate() {
        if i > 0 {
            separated.push(text(separator));
        }
        separated.push(doc.clone());
    }
    bracketed_layout(&sequence(separated), open_delimiter, close_delimiter)
}
