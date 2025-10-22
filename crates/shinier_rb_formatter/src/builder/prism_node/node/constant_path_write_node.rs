use crate::builder::Buildable;
use crate::document::Document;
use crate::helper::build_write::build_write;
use ruby_prism::ConstantPathWriteNode;

pub fn build_node(node: Option<&ConstantPathWriteNode>) -> Document {
    let node = node.unwrap();
    let target = node.target();
    let value = node.value();
    build_write(target.as_node().build(), value.build())
}
