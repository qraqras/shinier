use crate::builder::BuildableList;
use crate::builder::builder::*;
use crate::document::*;
use crate::keyword::COMMA;
use ruby_prism::KeywordHashNode;

pub fn build_node(node: Option<&KeywordHashNode>) -> Doc {
    let node = node.unwrap();
    let elements = node.elements();
    elements.build(array(&[string(COMMA), line()]), array)
}
