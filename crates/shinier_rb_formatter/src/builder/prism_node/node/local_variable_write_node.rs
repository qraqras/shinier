use crate::builder::Buildable;
use crate::doc::Doc;
use crate::helper::build_write::build_write;
use ruby_prism::LocalVariableWriteNode;

pub fn build_node(node: Option<&LocalVariableWriteNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    build_write(name.build(), value.build())
}
