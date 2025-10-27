use crate::BuildPrismNode;
use crate::builder::builder::{array, group, hardline, indent, line, space, string};
use crate::document::Document;
use crate::keyword::{END, UNLESS};
use ruby_prism::Comments;
use ruby_prism::UnlessNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&UnlessNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let predicate = node.predicate();
    let statements = node.statements();
    let else_clause = node.else_clause();
    group(array(&[
        string(UNLESS),
        space(),
        predicate.build(comments),
        indent(statements.build_with(comments, Some(hardline()), None)),
        else_clause.build_with(comments, Some(hardline()), None),
        line(),
        string(END),
    ]))
}
