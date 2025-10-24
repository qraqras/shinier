use crate::buildable::Buildable;
use crate::document::Document;
use ruby_prism::ImplicitNode;

pub fn build_node(node: Option<&ImplicitNode>) -> Document {
    let node = node.unwrap();
    let value = node.value();
    value.build()
}
