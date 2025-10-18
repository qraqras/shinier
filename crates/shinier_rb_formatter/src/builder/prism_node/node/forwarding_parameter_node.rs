use crate::doc::{Doc, text};
use crate::keyword::TRIPLE_DOT;
use ruby_prism::ForwardingParameterNode;

pub fn build_node(node: Option<&ForwardingParameterNode>) -> Doc {
    let _node = node.unwrap();
    text(TRIPLE_DOT)
}
