use crate::builder::build;
use crate::doc::Doc;
use ruby_prism::NodeList;

pub fn separate<T, U>(items: T, separator: U) -> Vec<U>
where
    T: Iterator<Item = U>,
    U: Clone,
{
    let mut vec = Vec::new();
    for (i, item) in items.enumerate() {
        if i > 0 {
            vec.push(separator.clone());
        }
        vec.push(item);
    }
    vec
}

pub fn separate_nodelist(nodelist: &NodeList, separator: &Doc) -> Vec<Doc> {
    separate(nodelist.iter().map(|node| build(&node)), separator.clone())
}

pub fn separate_docs(docs: &[Doc], separator: &Doc) -> Vec<Doc> {
    let mut vec = Vec::with_capacity(docs.len().saturating_mul(2).saturating_sub(1));
    let mut should_separator = false;
    for doc in docs.iter() {
        match doc {
            Doc::None(_) => {}
            _ => {
                if should_separator {
                    vec.push(separator.clone());
                }
                vec.push(doc.clone());
                should_separator = true;
            }
        }
    }
    vec
}
