use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::builder::builder::{array, group, indent, softline, string};
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::PARENTHESES;
use ruby_prism::ParenthesesNode;
use std::collections::HashMap;

pub fn build_node(node: Option<&ParenthesesNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    let node = node.unwrap();
    let body = node.body();
    group(array(&[
        string(PARENTHESES.0),
        indent(array(&[softline(), body.build(comments)])),
        softline(),
        string(PARENTHESES.1),
    ]))
}
