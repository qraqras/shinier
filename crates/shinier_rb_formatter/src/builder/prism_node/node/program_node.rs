use crate::document::Document;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::Comments;
use ruby_prism::ProgramNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&ProgramNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let statements = node.statements();
    statements.as_node().build(comments)
}
