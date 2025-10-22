use crate::builder::Buildable;
use crate::document::*;
use ruby_prism::GlobalVariableReadNode;

pub fn build_node(node: Option<&GlobalVariableReadNode>) -> Doc {
    let node = node.unwrap();
    node.name().build()
}
