use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::builder::builder::*;
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::{END, WHILE};
use ruby_prism::WhileNode;
use std::collections::HashMap;

pub fn build_node(node: Option<&WhileNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    let node = node.unwrap();
    let is_begin_modifier = node.is_begin_modifier();
    let predicate = node.predicate();
    let statements = node.statements();
    match is_begin_modifier {
        true => group(array(&[
            statements.build(comments),
            space(),
            string(WHILE),
            space(),
            predicate.build(comments),
        ])),
        false => group(array(&[
            string(WHILE),
            space(),
            predicate.build(comments),
            indent(array(&[hardline(), statements.build(comments)])),
            line(),
            string(END),
        ])),
    }
}
