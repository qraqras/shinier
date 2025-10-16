use crate::builder::Buildable;
use crate::doc::Doc;
use crate::helper::build_write::build_write;
use ruby_prism::ConstantPathWriteNode;

pub fn build_node(node: Option<&ConstantPathWriteNode>) -> Doc {
    let node = node.unwrap();
    let target = node.target();
    let value = node.value();
    build_write(target.as_node().build(), value.build())
}
