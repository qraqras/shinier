use crate::builder::Buildable;
use crate::document::*;
use ruby_prism::LocalVariableReadNode;

pub fn build_node(node: Option<&LocalVariableReadNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    name.build()
}
