use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, line, string};
use crate::document::Document;
use crate::helper::build_index::build_index;
use crate::helper::build_write::build_logical_write;
use crate::helper::separate_docs::separate_docs;
use crate::keyword::{COMMA, LogicalOperator};
use ruby_prism::IndexOrWriteNode;

impl<'sh> Build for Option<&IndexOrWriteNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&IndexOrWriteNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let receiver = node.receiver();
    let arguments = node.arguments();
    let block = node.block();
    let value = node.value();

    let name = array(&[build_index(
        receiver.as_ref(),
        &separate_docs(
            &[
                arguments.as_ref().build(context),
                block.as_ref().build(context),
            ],
            array(&[string(COMMA), line()]),
        ),
        context,
    )]);
    build_logical_write(name, value.build(context), LogicalOperator::Or)
}
