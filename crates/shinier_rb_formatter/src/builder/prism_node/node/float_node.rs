use crate::buildable::Buildable;
use crate::document::*;
use ruby_prism::FloatNode;

pub fn build_node(node: Option<&FloatNode>) -> Doc {
    let node = node.unwrap();
    let location = node.location();
    location.build()
}
