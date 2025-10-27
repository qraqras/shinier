use crate::BuildPrismNode;
use crate::document::Document;
use crate::helper::build_write::build_write;
use ruby_prism::Comments;
use ruby_prism::GlobalVariableWriteNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&GlobalVariableWriteNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    build_write(name.build(comments), value.build(comments))
}
