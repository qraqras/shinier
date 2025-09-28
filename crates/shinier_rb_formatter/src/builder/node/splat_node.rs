use crate::builder::builder;
use crate::doc::{Doc, sequence, text};
use ruby_prism::SplatNode;

pub fn print(node: &SplatNode) -> Doc {
    if let Some(node) = node.expression() {
        return sequence(vec![text("*"), builder::build(&node)]);
    }
    Doc::default()
}
