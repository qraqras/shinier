use crate::builder::Buildable;
use crate::doc::{Doc, line, sequence, text};
use crate::helper::build_index::build_index;
use crate::helper::build_write::build_logical_write;
use crate::keyword::{COMMA, LogicalOperator};
use crate::layout::separate_docs;
use ruby_prism::IndexAndWriteNode;

pub fn build_node(node: Option<&IndexAndWriteNode>) -> Doc {
    let node = node.unwrap();
    let is_safe_navigation = node.is_safe_navigation();
    let receiver = node.receiver();
    let arguments = node.arguments();
    let block = node.block();
    let value = node.value();

    let name = sequence(&[build_index(
        receiver.as_ref(),
        &separate_docs(
            &[arguments.build(), block.build()],
            &sequence(&[text(COMMA), line()]),
        ),
        is_safe_navigation,
    )]);

    build_logical_write(name, value.build(), LogicalOperator::And)
}
