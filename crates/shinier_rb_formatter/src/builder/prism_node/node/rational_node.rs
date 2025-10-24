use crate::buildable::Buildable;
use crate::document::Document;
use ruby_prism::RationalNode;

pub fn build_node(node: Option<&RationalNode>) -> Document {
    let node = node.unwrap();
    let location = node.location();
    location.build()
}
