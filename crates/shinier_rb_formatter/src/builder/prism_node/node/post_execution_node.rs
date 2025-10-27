use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::builder::builder::{array, group, indent, line, space, string};
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::{BRACES, POST_EXECUTION};
use ruby_prism::PostExecutionNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(node: Option<&PostExecutionNode>, comments: &mut Peekable<Comments>, option: Option<&HashMap<&str, bool>>) -> Document {
    let node = node.unwrap();
    let statements = node.statements();
    group(array(&[
        string(POST_EXECUTION),
        space(),
        string(BRACES.0),
        indent(array(&[line(), statements.build(comments)])),
        line(),
        string(BRACES.1),
    ]))
}
