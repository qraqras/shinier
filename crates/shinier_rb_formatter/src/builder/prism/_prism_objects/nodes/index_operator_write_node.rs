use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, line, string};
use crate::builder::prism::helper::layout::build_index;
use crate::builder::prism::helper::layout::build_operator_write;
use crate::builder::prism::helper::layout::separate_docs;
use crate::document::Document;
use crate::keyword::COMMA;
use ruby_prism::IndexOperatorWriteNode;

impl<'sh> Build for IndexOperatorWriteNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &IndexOperatorWriteNode, context: &mut BuildContext) -> Document {
    let receiver = node.receiver();
    let arguments = node.arguments();
    let block = node.block();
    let value = node.value();
    let binary_operator = node.binary_operator();

    let name = array(&[build_index(
        receiver.as_ref(),
        &separate_docs(
            &[arguments.build(context), block.build(context)],
            array(&[string(COMMA), line()]),
        ),
        context,
    )]);
    build_operator_write(name, value.build(context), binary_operator.build(context))
}
