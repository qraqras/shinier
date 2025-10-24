use crate::buildable::Buildable;
use crate::document::Document;
use ruby_prism::*;

pub fn build_node(node: Option<&ItParametersNode>) -> Document {
    let node = node.unwrap();
    node.as_node().build()
}
