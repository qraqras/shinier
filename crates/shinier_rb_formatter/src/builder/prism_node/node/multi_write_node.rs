use crate::builder::builder::{array, group, line, space, string};
use crate::document::Document;
use crate::helper::build_rest::build_rest;
use crate::keyword::{COMMA, WRITE_OPERATOR};
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::Comments;
use ruby_prism::MultiWriteNode;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&MultiWriteNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let lefts = node.lefts();
    let rest = node.rest();
    let rights = node.rights();
    let value = node.value();

    let separator = array(&[string(COMMA), line()]);
    group(array(&[
        build_rest(lefts, rest, rights, &separator, comments),
        space(),
        string(WRITE_OPERATOR),
        line(),
        value.build(comments),
    ]))
}
