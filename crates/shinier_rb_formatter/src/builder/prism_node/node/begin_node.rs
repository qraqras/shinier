use crate::builder::builder::{array, group, hardline, indent, string};
use crate::document::Document;
use crate::keyword::{BEGIN, END};
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::BeginNode;
use ruby_prism::Comments;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&BeginNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let statements = node.statements();
    let rescue_clause = node.rescue_clause();
    let else_clause = node.else_clause();
    let ensure_clause = node.ensure_clause();
    group(array(&[
        string(BEGIN),
        indent(array(&[statements.build_with(
            comments,
            Some(hardline()),
            None,
        )])),
        rescue_clause.build_with(comments, Some(hardline()), None),
        else_clause.build_with(comments, Some(hardline()), None),
        ensure_clause.build_with(comments, Some(hardline()), None),
        hardline(),
        string(END),
    ]))
}
