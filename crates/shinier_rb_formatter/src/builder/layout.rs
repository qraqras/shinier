use crate::builder::build;
use crate::doc::{Docs, line, text};
use ruby_prism::NodeList;

pub fn separate(nodelist: &NodeList, separator: &str) -> Docs {
    let mut parts = Vec::new();
    for (i, node) in nodelist.iter().enumerate() {
        if i > 0 {
            parts.push(text(separator));
            parts.push(line());
        }
        parts.push(build(&node));
    }
    parts
}
