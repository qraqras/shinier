use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::builder::builder::{array, none, string};
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::ASTERISK;
use ruby_prism::SplatNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(node: Option<&SplatNode>, comments: &mut Peekable<Comments>, option: Option<&HashMap<&str, bool>>) -> Document {
    match node {
        Some(node) => {
            let expression = node.expression();
            array(&[string(ASTERISK), expression.build(comments)])
        }
        None => none(),
    }
}
