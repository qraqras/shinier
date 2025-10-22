use crate::document::Doc;

pub fn separate_docs(docs: &[Doc], separator: Doc) -> Vec<Doc> {
    let mut vec = Vec::with_capacity(docs.len().saturating_mul(2).saturating_sub(1));
    let mut should_separate = false;
    for doc in docs.iter() {
        if matches!(doc, Doc::None) {
            if should_separate {
                vec.push(separator.clone());
            }
            vec.push(doc.clone());
            should_separate = true;
        }
    }
    vec
}
