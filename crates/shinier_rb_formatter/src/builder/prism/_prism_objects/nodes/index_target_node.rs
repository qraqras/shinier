use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, line, string};
use crate::builder::helper::layout::build_index;
use crate::builder::helper::layout::separate_docs;
use crate::document::Document;
use crate::keyword::COMMA;
use ruby_prism::IndexTargetNode;

impl<'sh> Build for IndexTargetNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &IndexTargetNode, context: &mut BuildContext) -> Document {
    let receiver = node.receiver();
    let arguments = node.arguments();
    let block = node.block();
    group(array(&[build_index(
        Some(&receiver),
        &separate_docs(
            &[arguments.build(context), block.build(context)],
            array(&[string(COMMA), line()]),
        ),
        context,
    )]))
}
