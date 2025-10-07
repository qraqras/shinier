use crate::builder::builder;
use crate::doc::{Doc, sequence, text};
use ruby_prism::SplatNode;

pub fn build_node(node: &SplatNode) -> Doc {
    if let Some(node) = node.expression() {
        return sequence(&[text("*"), builder::build(&node)]);
    }
    Doc::default()
}
