use crate::buildable::Buildable;
use crate::builder::helper::build_write::build_write;
use crate::document::Document;
use ruby_prism::InstanceVariableWriteNode;

pub fn build_node(node: Option<&InstanceVariableWriteNode>) -> Document {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    build_write(name.build(), value.build())
}
