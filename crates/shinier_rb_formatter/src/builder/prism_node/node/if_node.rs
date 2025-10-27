use crate::builder::builder::{array, group, hardline, indent, space, string};
use crate::document::Document;
use crate::keyword::{END, IF};
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::Comments;
use ruby_prism::IfNode;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&IfNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let predicate = node.predicate();
    let statements = node.statements();
    let subsequent = node.subsequent();
    group(array(&[
        string(IF),
        space(),
        predicate.build(comments),
        indent(statements.build_with(comments, Some(hardline()), None)),
        subsequent.build_with(comments, Some(hardline()), None),
        hardline(),
        string(END),
    ]))
}
