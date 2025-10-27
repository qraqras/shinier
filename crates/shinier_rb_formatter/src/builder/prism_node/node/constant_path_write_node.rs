use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::document::Document;
use ruby_prism::Comments;
use crate::helper::build_write::build_write;
use ruby_prism::ConstantPathWriteNode;
use std::collections::HashMap;

pub fn build_node(node: Option<&ConstantPathWriteNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    let node = node.unwrap();
    let target = node.target();
    let value = node.value();
    build_write(target.as_node().build(comments), value.build(comments))
}
