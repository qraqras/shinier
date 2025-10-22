use crate::builder::Buildable;
use crate::document::Document;
use ruby_prism::ConstantReadNode;

pub fn build_node(node: Option<&ConstantReadNode>) -> Document {
    let node = node.unwrap();
    node.name().build()
}
