use crate::builder::node::arguments_node;
use crate::doc::{Doc, line, sequence, text};
use crate::keyword::BREAK;
use ruby_prism::BreakNode;

pub fn build_node(node: Option<&BreakNode>) -> Doc {
    let node = node.unwrap();
    let arguments = node.arguments();
    sequence(&[
        text(BREAK),
        line(),
        arguments_node::build_node(arguments.as_ref()),
    ])
}
