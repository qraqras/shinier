use crate::buildable::Buildable;
use crate::document::Document;
use ruby_prism::FloatNode;

pub fn build_node(node: Option<&FloatNode>) -> Document {
    let node = node.unwrap();
    let location = node.location();
    location.build()
}
