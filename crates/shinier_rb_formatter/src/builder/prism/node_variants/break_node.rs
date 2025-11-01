use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, line, string};
use crate::document::Document;
use crate::keyword::BREAK;
use ruby_prism::BreakNode;

impl<'sh> Build for BreakNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &BreakNode, context: &mut BuildContext) -> Document {
    let arguments = node.arguments();
    group(array(&[
        string(BREAK),
        arguments.build_with(context, Some(line()), None),
    ]))
}
