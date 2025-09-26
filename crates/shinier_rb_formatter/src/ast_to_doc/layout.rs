use crate::doc::{Docs, line, text};

pub fn separate(docs: &Docs, separator: &str) -> Docs {
    let len = docs.len();
    let mut parts = Vec::with_capacity(len * 3);
    for (i, doc) in docs.iter().enumerate() {
        parts.push(doc.clone());
        if i < (len - 1) {
            parts.push(text(separator));
            parts.push(line());
        }
    }
    parts
}
