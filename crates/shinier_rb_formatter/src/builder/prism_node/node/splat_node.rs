use crate::builder::Buildable;
use crate::builder::builder::{array, none, string};
use crate::document::Document;
use crate::keyword::ASTERISK;
use ruby_prism::SplatNode;

pub fn build_node(node: Option<&SplatNode>) -> Document {
    match node {
        Some(node) => {
            let expression = node.expression();
            array(&[string(ASTERISK), expression.build()])
        }
        None => none(),
    }
}
