use crate::BuildPrismNode;
use crate::builder::builder::*;
use crate::document::Document;
use crate::keyword::{BRACES, PRE_EXECUTION};
use ruby_prism::Comments;
use ruby_prism::*;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&PreExecutionNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let statements = node.statements();
    group(array(&[
        string(PRE_EXECUTION),
        space(),
        group(array(&[
            string(BRACES.0),
            indent(array(&[line(), statements.build(comments)])),
            line(),
            string(BRACES.1),
        ])),
    ]))
}
