use crate::builder::Buildable;
use crate::builder::builder::{array, line, string};
use crate::document::Document;
use crate::helper::build_index::build_index;
use crate::helper::build_write::build_operator_write;
use crate::helper::separate_docs::separate_docs;
use crate::keyword::COMMA;
use ruby_prism::IndexOperatorWriteNode;

pub fn build_node(node: Option<&IndexOperatorWriteNode>) -> Document {
    let node = node.unwrap();
    let is_safe_navigation = node.is_safe_navigation();
    let receiver = node.receiver();
    let arguments = node.arguments();
    let block = node.block();
    let value = node.value();
    let binary_operator = node.binary_operator();

    let name = array(&[build_index(
        receiver.as_ref(),
        &separate_docs(
            &[arguments.build(), block.build()],
            array(&[string(COMMA), line()]),
        ),
        is_safe_navigation,
    )]);
    build_operator_write(name, value.build(), binary_operator.build())
}
