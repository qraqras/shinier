use crate::document::Document;

pub fn separate_docs(docs: &[Document], separator: Document) -> Vec<Document> {
    let mut vec = Vec::with_capacity(docs.len().saturating_mul(2).saturating_sub(1));
    let mut should_separate = false;
    for doc in docs.iter() {
        match doc {
            Document::None => {}
            _ => {
                if should_separate {
                    vec.push(separator.clone());
                }
                vec.push(doc.clone());
                should_separate = true;
            }
        }
    }
    vec
}
