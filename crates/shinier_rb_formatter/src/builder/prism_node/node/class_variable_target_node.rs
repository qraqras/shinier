use crate::builder::Buildable;
use crate::document::*;
use ruby_prism::ClassVariableTargetNode;

pub fn build_node(node: Option<&ClassVariableTargetNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    name.build()
}
