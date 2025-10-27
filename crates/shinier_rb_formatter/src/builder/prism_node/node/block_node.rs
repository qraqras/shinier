use crate::builder::builder::{array, group, indent, line, none, string};
use crate::document::Document;
use crate::keyword::{BRACES, PIPE};
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::BlockNode;
use ruby_prism::Comments;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&BlockNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    match node {
        Some(node) => {
            let parameters = node.parameters();
            let body = node.body();
            group(array(&[
                string(BRACES.0),
                indent(array(&[
                    group(parameters.build_with(
                        comments,
                        Some(array(&[line(), string(PIPE)])),
                        Some(string(PIPE)),
                    )),
                    body.build_with(comments, Some(line()), None),
                ])),
                line(),
                string(BRACES.1),
            ]))
        }
        None => none(),
    }
}
