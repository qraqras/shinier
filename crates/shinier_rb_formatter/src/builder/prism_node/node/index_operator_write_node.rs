use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, line, string};
use crate::document::Document;
use crate::helper::build_index::build_index;
use crate::helper::build_write::build_operator_write;
use crate::helper::separate_docs::separate_docs;
use crate::keyword::COMMA;
use ruby_prism::IndexOperatorWriteNode;

pub fn build_node(node: Option<&IndexOperatorWriteNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
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
