use crate::builder::builder::{array, line, string};
use crate::document::Document;
use crate::helper::build_index::build_index;
use crate::helper::build_write::build_operator_write;
use crate::helper::separate_docs::separate_docs;
use crate::keyword::COMMA;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::Comments;
use ruby_prism::IndexOperatorWriteNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&IndexOperatorWriteNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let receiver = node.receiver();
    let arguments = node.arguments();
    let block = node.block();
    let value = node.value();
    let binary_operator = node.binary_operator();

    let name = array(&[build_index(
        receiver.as_ref(),
        &separate_docs(
            &[arguments.build(comments), block.build(comments)],
            array(&[string(COMMA), line()]),
        ),
        comments,
    )]);
    build_operator_write(name, value.build(comments), binary_operator.build(comments))
}
