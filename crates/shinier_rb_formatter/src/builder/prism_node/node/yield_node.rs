use crate::BuildPrismNode;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::YIELD;
use ruby_prism::Comments;
use ruby_prism::YieldNode;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&YieldNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    group(array(&[
        string(YIELD),
        space(),
        node.arguments().build(comments),
    ]))
}
