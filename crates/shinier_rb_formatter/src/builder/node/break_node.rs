use crate::builder::node::arguments_node;
use crate::doc::{Doc, line, sequence, text};
use ruby_prism::BreakNode;

const BREAK_KEYWORD: &str = "break";

pub fn build_node(node: Option<&BreakNode>) -> Doc {
    let node = node.unwrap();
    let arguments = node.arguments();
    sequence(&[
        text(BREAK_KEYWORD),
        line(),
        arguments_node::build_node(arguments.as_ref()),
    ])
}
