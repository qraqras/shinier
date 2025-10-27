use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::document::Document;
use ruby_prism::Comments;
use ruby_prism::ProgramNode;
use std::collections::HashMap;

pub fn build_node(node: Option<&ProgramNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    let node = node.unwrap();
    let statements = node.statements();
    statements.as_node().build(comments)
}
