use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::ALTERNATION;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::AlternationPatternNode;
use ruby_prism::Comments;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&AlternationPatternNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let left = node.left();
    let right = node.right();
    group(array(&[
        left.build(comments),
        space(),
        string(ALTERNATION),
        space(),
        right.build(comments),
    ]))
}
