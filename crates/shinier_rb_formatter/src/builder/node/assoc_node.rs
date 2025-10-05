use crate::builder::build;
use crate::doc::{Doc, group, text};
use ruby_prism::AssocNode;

const SEPARATER: &str = ": ";

pub fn build_node(node: &AssocNode) -> Doc {
    let key = build(&node.key());
    let value = build(&node.value());
    return group(vec![key, text(SEPARATER), value]);
}
