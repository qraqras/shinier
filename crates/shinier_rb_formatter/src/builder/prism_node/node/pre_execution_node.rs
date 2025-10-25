use crate::buildable::Buildable;
use crate::builder::builder::*;
use crate::document::Document;
use crate::keyword::{BRACES, PRE_EXECUTION};
use ruby_prism::*;

pub fn build_node(node: Option<&PreExecutionNode>) -> Document {
    let node = node.unwrap();
    let statements = node.statements();
    group(array(&[
        string(PRE_EXECUTION),
        space(),
        group(array(&[
            string(BRACES.0),
            indent(array(&[line(), statements.build()])),
            line(),
            string(BRACES.1),
        ])),
    ]))
}
