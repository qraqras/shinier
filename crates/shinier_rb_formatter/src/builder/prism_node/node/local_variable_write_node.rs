use crate::builder::Buildable;
use crate::doc::Doc;
use crate::helper::write_node::build_write_node;
use ruby_prism::LocalVariableWriteNode;

pub fn build_node(node: Option<&LocalVariableWriteNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    build_write_node(name.build(), value.build())
}
