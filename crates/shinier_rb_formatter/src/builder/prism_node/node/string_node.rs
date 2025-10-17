use crate::buildable::Buildable;
use crate::doc::Doc;
use ruby_prism::StringNode;

pub fn build_node(node: Option<&StringNode>) -> Doc {
    let node = node.unwrap();
    node.location().build()
}
