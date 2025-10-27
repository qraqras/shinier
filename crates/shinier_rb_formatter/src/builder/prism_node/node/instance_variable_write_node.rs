use crate::BuildPrismNode;
use crate::builder::helper::build_write::build_write;
use crate::document::Document;
use ruby_prism::Comments;
use ruby_prism::InstanceVariableWriteNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&InstanceVariableWriteNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    build_write(name.build(comments), value.build(comments))
}
