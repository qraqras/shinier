use crate::builder::builder::{array, group, line, string};
use crate::document::Document;
use crate::keyword::BREAK;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::BreakNode;
use ruby_prism::Comments;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&BreakNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let arguments = node.arguments();
    group(array(&[
        string(BREAK),
        arguments.build_with(comments, Some(line()), None),
    ]))
}
