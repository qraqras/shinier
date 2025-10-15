use crate::builder::Buildable;
use crate::doc::Doc;
use crate::helper::build_write_pattern::build_write_pattern;
use ruby_prism::LocalVariableWriteNode;

pub fn build_node(node: Option<&LocalVariableWriteNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    build_write_pattern(name.build(), value.build())
}
