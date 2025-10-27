use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, indent, line, space, string};
use crate::document::Document;
use crate::keyword::{BRACES, PRE_EXECUTION};
use ruby_prism::*;

pub fn build_node(node: Option<&PreExecutionNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let statements = node.statements();
    group(array(&[
        string(PRE_EXECUTION),
        space(),
        group(array(&[
            string(BRACES.0),
            indent(array(&[line(), statements.build(context)])),
            line(),
            string(BRACES.1),
        ])),
    ]))
}
