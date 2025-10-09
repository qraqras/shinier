use crate::builder::build;
use crate::doc::{Doc, line, sequence, text};
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

pub fn separate_nodelist(nodelist: &NodeList, separator: &str) -> Vec<Doc> {
    separate(
        nodelist.iter().map(|node| build(&node)),
        sequence(&[text(separator), line()]),
    )
}

pub fn separate_docs(docs: &[Doc]) -> Vec<Doc> {
    const SEPARATOR: &str = ",";
    let mut vec = Vec::with_capacity(docs.len().saturating_mul(2).saturating_sub(1));
    let mut should_separator = false;
    for doc in docs.iter() {
        match doc {
            Doc::None(_) => {}
            _ => {
                if should_separator {
                    vec.push(sequence(&[text(SEPARATOR), line()]));
                }
                vec.push(doc.clone());
                should_separator = true;
            }
        }
    }
    vec
}
