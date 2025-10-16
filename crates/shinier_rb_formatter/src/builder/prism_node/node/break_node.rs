use crate::buildable::Buildable;
use crate::doc::{Doc, line, sequence, text};
use crate::keyword::BREAK;
use ruby_prism::BreakNode;

pub fn build_node(node: Option<&BreakNode>) -> Doc {
    let node = node.unwrap();
    let arguments = node.arguments();
    sequence(&[text(BREAK), arguments.build_with(Some(line()), None)])
}
