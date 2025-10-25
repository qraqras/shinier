use crate::builder::Buildable;
use crate::builder::builder::{array, group, indent, line, space, string};
use crate::document::Document;
use crate::keyword::{BRACES, POST_EXECUTION};
use ruby_prism::PostExecutionNode;

pub fn build_node(node: Option<&PostExecutionNode>) -> Document {
    let node = node.unwrap();
    let statements = node.statements();
    group(array(&[
        string(POST_EXECUTION),
        space(),
        string(BRACES.0),
        indent(array(&[line(), statements.build()])),
        line(),
        string(BRACES.1),
    ]))
}
