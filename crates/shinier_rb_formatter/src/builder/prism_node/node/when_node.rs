use crate::builder::builder::{array, group, hardline, indent, space, string};
use crate::document::Document;
use crate::keyword::WHEN;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::Comments;
use ruby_prism::WhenNode;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&WhenNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let conditions = node.conditions();
    let statements = node.statements();
    group(array(&[
        string(WHEN),
        space(),
        conditions.build(&hardline(), comments),
        indent(statements.build_with(comments, Some(hardline()), None)),
    ]))
}
