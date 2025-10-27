use crate::builder::builder::{array, group, indent, line, softline, string};
use crate::document::Document;
use crate::helper::separate_docs::separate_docs;
use crate::keyword::{BRACKETS, COMMA};
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::ArrayPatternNode;
use ruby_prism::Comments;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&ArrayPatternNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let constant = node.constant();
    let requireds = node.requireds();
    let rest = node.rest();
    let posts = node.posts();

    let separator = array(&[string(COMMA), line()]);

    let separated_requireds = requireds.build(&separator, comments);
    let separated_posts = posts.build(&separator, comments);

    group(array(&[
        constant.build(comments),
        string(BRACKETS.0),
        indent(array(&[
            softline(),
            group(array(&separate_docs(
                &[separated_requireds, rest.build(comments), separated_posts],
                separator.clone(),
            ))),
        ])),
        softline(),
        string(BRACKETS.1),
    ]))
}
