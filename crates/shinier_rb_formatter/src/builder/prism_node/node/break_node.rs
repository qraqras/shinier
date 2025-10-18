use crate::buildable::Buildable;
use crate::doc::{Doc, group, line, text};
use crate::keyword::BREAK;
use ruby_prism::BreakNode;

pub fn build_node(node: Option<&BreakNode>) -> Doc {
    let node = node.unwrap();
    let arguments = node.arguments();
    group(&[text(BREAK), arguments.build_with(Some(line()), None)])
}
