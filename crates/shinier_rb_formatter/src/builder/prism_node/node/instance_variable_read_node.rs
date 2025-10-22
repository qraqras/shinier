use crate::buildable::Buildable;
use crate::document::*;
use ruby_prism::InstanceVariableReadNode;

pub fn build_node(node: Option<&InstanceVariableReadNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    name.build()
}
