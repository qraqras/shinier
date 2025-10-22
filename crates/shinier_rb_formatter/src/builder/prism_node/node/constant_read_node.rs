use crate::builder::Buildable;
use crate::document::*;
use ruby_prism::ConstantReadNode;

pub fn build_node(node: Option<&ConstantReadNode>) -> Doc {
    let node = node.unwrap();
    node.name().build()
}
