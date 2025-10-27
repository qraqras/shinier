use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::document::Document;
use ruby_prism::Comments;
use crate::helper::build_write::build_write;
use ruby_prism::ClassVariableWriteNode;
use std::collections::HashMap;

pub fn build_node(node: Option<&ClassVariableWriteNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    build_write(name.build(comments), value.build(comments))
}
