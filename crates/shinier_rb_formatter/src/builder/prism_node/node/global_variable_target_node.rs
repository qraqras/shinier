use crate::BuildPrismNode;
use crate::document::Document;
use ruby_prism::Comments;
use ruby_prism::GlobalVariableTargetNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&GlobalVariableTargetNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let name = node.name();
    name.build(comments)
}
