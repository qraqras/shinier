use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::builder::builder::{array, group, indent, line, space, string};
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::ROCKET;
use ruby_prism::MatchRequiredNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(node: Option<&MatchRequiredNode>, comments: &mut Peekable<Comments>, option: Option<&HashMap<&str, bool>>) -> Document {
    let node = node.unwrap();
    let value = node.value();
    let pattern = node.pattern();
    group(array(&[
        value.build(comments),
        space(),
        string(ROCKET),
        indent(array(&[line(), pattern.build(comments)])),
    ]))
}
