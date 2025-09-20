use crate::doc::*;

pub fn separated(docs: &Docs, sep: &str) -> Doc {
    if docs.is_empty() {
        return Doc::default();
    }
    let mut result: Vec<Doc> = Vec::with_capacity(docs.len() * 2 - 1);
    let mut iter = docs.iter();
    if let Some(first) = iter.next() {
        result.push(first.clone());
    }
    for doc in iter {
        result.push(text(sep.to_string()));
        result.push(doc.clone());
    }
    sequence(result)
}

pub fn array(docs: &Docs) -> Doc {
    if docs.is_empty() {
        text("[]".to_string())
    } else {
        sequence(vec![
            text("[".to_string()),
            separated(docs, ", "),
            text("]".to_string()),
        ])
    }
}
