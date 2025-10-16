use crate::builder::BuildableList;
use crate::doc::{Doc, group, line, sequence, text};
use crate::keyword::COMMA;
use ruby_prism::KeywordHashNode;

pub fn build_node(node: Option<&KeywordHashNode>) -> Doc {
    let node = node.unwrap();
    let elements = node.elements();
    elements.build(sequence(&[text(COMMA), line()]), group)
}
