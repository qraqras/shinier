use crate::buildable::Buildable;
use crate::document::Document;
use ruby_prism::InstanceVariableTargetNode;

pub fn build_node(node: Option<&InstanceVariableTargetNode>) -> Document {
    let node = node.unwrap();
    let name = node.name();
    name.build()
}
