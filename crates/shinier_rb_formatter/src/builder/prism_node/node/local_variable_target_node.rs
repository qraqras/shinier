use crate::builder::Buildable;
use crate::document::*;
use ruby_prism::LocalVariableTargetNode;

pub fn build_node(node: Option<&LocalVariableTargetNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    name.build()
}
