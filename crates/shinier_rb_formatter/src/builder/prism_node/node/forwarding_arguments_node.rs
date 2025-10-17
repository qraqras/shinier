use crate::doc::{Doc, text};
use crate::keyword::TRIPLE_DOT;
use ruby_prism::ForwardingArgumentsNode;

pub fn build_node(node: Option<&ForwardingArgumentsNode>) -> Doc {
    let _node = node.unwrap();
    text(TRIPLE_DOT)
}
