use crate::buildable::Buildable;
use crate::document::*;
use ruby_prism::StringNode;

pub fn build_node(node: Option<&StringNode>) -> Doc {
    let node = node.unwrap();
    node.location().build()
}
