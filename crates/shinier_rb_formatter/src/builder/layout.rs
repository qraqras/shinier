use crate::builder::build;
use crate::doc::{Doc, line, text};
use ruby_prism::NodeList;

pub fn separate(nodelist: &NodeList, separator: &str) -> Vec<Doc> {
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
