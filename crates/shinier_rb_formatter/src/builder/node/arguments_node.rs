use crate::builder::layout::separate;
use crate::doc::{Doc, group, indent, softline, text};
use ruby_prism::ArgumentsNode;

const SEPARATER: &str = ",";

pub fn build_node(node: &ArgumentsNode) -> Doc {
    group(separate(&node.arguments(), SEPARATER))
}
