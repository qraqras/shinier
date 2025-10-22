use crate::buildable::Buildable;
use crate::builder::builder::*;
use crate::document::Document;
use crate::keyword::{DEFINED, PARENTHESES};
use ruby_prism::DefinedNode;

pub fn build_node(node: Option<&DefinedNode>) -> Document {
    let node = node.unwrap();
    let value = node.value();
    group(array(&[
        string(DEFINED),
        string(PARENTHESES.0),
        value.build(),
        string(PARENTHESES.1),
    ]))
}
