use crate::builder::builder::{array, group, line, none, string};
use crate::document::Document;
use crate::keyword::COMMA;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::{ArgumentsNode, Comments};
use std::collections::HashMap;

pub fn build_node(
    node: Option<&ArgumentsNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    match node {
        Some(node) => {
            let arguments = node.arguments();
            let separator = array(&[string(COMMA), line()]);
            group(arguments.build(&separator, comments))
        }
        None => none(),
    }
}
