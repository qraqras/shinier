use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, indent, line, space, string};
use crate::document::Document;
use crate::keyword::{BRACES, POST_EXECUTION};
use ruby_prism::PostExecutionNode;

impl<'sh> Build for PostExecutionNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &PostExecutionNode, context: &mut BuildContext) -> Document {
    let statements = node.statements();
    group(array(&[
        string(POST_EXECUTION),
        space(),
        string(BRACES.0),
        indent(array(&[line(), statements.build(context)])),
        line(),
        string(BRACES.1),
    ]))
}
