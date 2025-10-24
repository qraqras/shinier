use crate::buildable::Buildable;
use crate::document::Document;
use crate::helper::build_write::build_write;
use ruby_prism::GlobalVariableWriteNode;

pub fn build_node(node: Option<&GlobalVariableWriteNode>) -> Document {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    build_write(name.build(), value.build())
}
