use crate::BuildPrismNode;
use crate::builder::builder::{array, group, hardline, indent, line, space, string};
use crate::document::Document;
use crate::keyword::{END, UNTIL};
use ruby_prism::Comments;
use ruby_prism::UntilNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&UntilNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let is_begin_modifier = node.is_begin_modifier();
    let predicate = node.predicate();
    let statements = node.statements();
    match is_begin_modifier {
        true => group(array(&[
            statements.build(comments),
            space(),
            string(UNTIL),
            space(),
            predicate.build(comments),
        ])),
        false => group(array(&[
            string(UNTIL),
            space(),
            predicate.build(comments),
            indent(array(&[hardline(), statements.build(comments)])),
            line(),
            string(END),
        ])),
    }
}
