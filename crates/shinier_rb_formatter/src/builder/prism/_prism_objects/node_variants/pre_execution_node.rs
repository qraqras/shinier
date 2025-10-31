use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, indent, line, space, string};
use crate::document::Document;
use crate::keyword::{BRACES, PRE_EXECUTION};
use ruby_prism::PreExecutionNode;

impl<'sh> Build for PreExecutionNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &PreExecutionNode, context: &mut BuildContext) -> Document {
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
