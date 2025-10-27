use crate::builder::builder::{array, group, hardline, space, string};
use crate::document::Document;
use crate::keyword::{CASE, END};
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::CaseMatchNode;
use ruby_prism::Comments;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&CaseMatchNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let predicate = node.predicate();
    let conditions = node.conditions();
    let else_clause = node.else_clause();
    group(array(&[
        string(CASE),
        predicate.build_with(comments, Some(space()), None),
        conditions.build_with(&hardline(), comments, Some(hardline()), None),
        else_clause.build_with(comments, Some(hardline()), None),
        hardline(),
        string(END),
    ]))
}
