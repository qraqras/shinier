use crate::BuildPrismNode;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::{DOUBLE_DOT, TRIPLE_DOT};
use ruby_prism::Comments;
use ruby_prism::FlipFlopNode;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&FlipFlopNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let is_exclude_end = node.is_exclude_end();
    let left = node.left();
    let right = node.right();
    group(array(&[
        left.build(comments),
        space(),
        match is_exclude_end {
            true => string(TRIPLE_DOT),
            false => string(DOUBLE_DOT),
        },
        space(),
        right.build(comments),
    ]))
}
