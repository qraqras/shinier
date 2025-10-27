use crate::builder::builder::{array, group, line, string};
use crate::builder::helper::build_index::build_index;
use crate::builder::helper::separate_docs::separate_docs;
use crate::document::Document;
use crate::keyword::COMMA;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::Comments;
use ruby_prism::IndexTargetNode;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&IndexTargetNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let receiver = node.receiver();
    let arguments = node.arguments();
    let block = node.block();
    group(array(&[build_index(
        Some(&receiver),
        &separate_docs(
            &[arguments.build(comments), block.build(comments)],
            array(&[string(COMMA), line()]),
        ),
        comments,
    )]))
}
