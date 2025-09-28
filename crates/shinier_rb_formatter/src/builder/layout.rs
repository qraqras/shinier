use crate::doc::{Docs, line, text};

pub fn separate(docs: &Docs, separator: &str) -> Docs {
    let mut parts = Vec::with_capacity(docs.len() * 3);
    for (i, doc) in docs.iter().enumerate() {
        if i > 0 {
            parts.push(text(separator));
            parts.push(line());
        }
        parts.push(doc.clone());
    }
    parts
}
