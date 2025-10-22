use crate::buildable::Buildable;
use crate::document::Document;
use ruby_prism::StringNode;

pub fn build_node(node: Option<&StringNode>) -> Document {
    let node = node.unwrap();
    node.location().build()
}
