use crate::buildable::Buildable;
use crate::document::Document;
use ruby_prism::InstanceVariableReadNode;

pub fn build_node(node: Option<&InstanceVariableReadNode>) -> Document {
    let node = node.unwrap();
    let name = node.name();
    name.build()
}
