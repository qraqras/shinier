use crate::builder::builder::{array, group, indent, line, softline, string};
use crate::builder::helper::separate_docs::separate_docs;
use crate::document::Document;
use crate::keyword::{COMMA, PARENTHESES};
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::Comments;
use ruby_prism::MultiTargetNode;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&MultiTargetNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let lefts = node.lefts();
    let rest = node.rest();
    let rights = node.rights();

    let separator = array(&[string(COMMA), line()]);
    group(array(&[
        string(PARENTHESES.0),
        indent(array(&[
            softline(),
            group(array(&separate_docs(
                &[
                    lefts.build(&separator, comments),
                    rest.build(comments),
                    rights.build(&separator, comments),
                ],
                separator.clone(),
            ))),
        ])),
        softline(),
        string(PARENTHESES.1),
    ]))
}
