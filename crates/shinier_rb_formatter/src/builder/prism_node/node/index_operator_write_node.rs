use crate::builder::Buildable;
use crate::doc::{Doc, line, sequence, text};
use crate::helper::build_index::build_index;
use crate::helper::build_write::build_operator_write;
use crate::keyword::COMMA;
use crate::layout::separate_docs;
use ruby_prism::IndexOperatorWriteNode;

pub fn build_node(node: Option<&IndexOperatorWriteNode>) -> Doc {
    let node = node.unwrap();
    let is_safe_navigation = node.is_safe_navigation();
    let receiver = node.receiver();
    let arguments = node.arguments();
    let block = node.block();
    let value = node.value();
    let binary_operator = node.binary_operator();

    let name = sequence(&[build_index(
        receiver.as_ref(),
        &separate_docs(
            &[arguments.build(), block.build()],
            &sequence(&[text(COMMA), line()]),
        ),
        is_safe_navigation,
    )]);
    build_operator_write(name, value.build(), binary_operator.build())
}
