use crate::builder::Buildable;
use crate::builder::builder::{array, line, string};
use crate::document::Document;
use crate::helper::build_index::build_index;
use crate::helper::build_write::build_logical_write;
use crate::helper::separate_docs::separate_docs;
use crate::keyword::{COMMA, LogicalOperator};
use ruby_prism::IndexOrWriteNode;

pub fn build_node(node: Option<&IndexOrWriteNode>) -> Document {
    let node = node.unwrap();
    let receiver = node.receiver();
    let arguments = node.arguments();
    let block = node.block();
    let value = node.value();

    let name = array(&[build_index(
        receiver.as_ref(),
        &separate_docs(
            &[arguments.build(), block.build()],
            array(&[string(COMMA), line()]),
        ),
    )]);
    build_logical_write(name, value.build(), LogicalOperator::Or)
}
