use crate::BuildPrismNode;
use crate::builder::builder::{array, group, hardline, indent, space, string};
use crate::document::Document;
use crate::keyword::IN;
use ruby_prism::Comments;
use ruby_prism::InNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&InNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let pattern = node.pattern();
    let statements = node.statements();
    group(array(&[
        string(IN),
        space(),
        pattern.build(comments),
        indent(statements.build_with(comments, Some(hardline()), None)),
    ]))
}
