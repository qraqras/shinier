use crate::builder::Buildable;
use crate::doc::{Doc, sequence, text};
use ruby_prism::SplatNode;

pub fn build_node(node: Option<&SplatNode>) -> Doc {
    let node = node.unwrap();
    if let Some(node) = node.expression() {
        return sequence(&[text("*"), node.build()]);
    }
    Doc::default()
}
