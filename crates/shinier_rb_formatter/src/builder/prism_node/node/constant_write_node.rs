use crate::builder::Buildable;
use crate::doc::{Doc, line, sequence, space, text};
use crate::keyword::WRITE_OPERATOR;
use ruby_prism::ConstantWriteNode;

pub fn build_node(node: Option<&ConstantWriteNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name().build();
    let value = node.value().build();
    sequence(&[name, space(), text(WRITE_OPERATOR), line(), value])
}
