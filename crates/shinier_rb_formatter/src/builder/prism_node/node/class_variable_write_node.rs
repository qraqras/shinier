use crate::builder::Buildable;
use crate::doc::Doc;
use crate::helper::build_write::build_write;
use ruby_prism::ClassVariableWriteNode;

pub fn build_node(node: Option<&ClassVariableWriteNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    build_write(name.build(), value.build())
}
