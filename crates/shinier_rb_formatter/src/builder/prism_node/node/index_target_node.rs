use crate::builder::Buildable;
use crate::builder::builder::{array, group, line, string};
use crate::builder::helper::build_index::build_index;
use crate::builder::helper::separate_docs::separate_docs;
use crate::document::Document;
use crate::keyword::COMMA;
use ruby_prism::IndexTargetNode;

pub fn build_node(node: Option<&IndexTargetNode>) -> Document {
    let node = node.unwrap();
    let receiver = node.receiver();
    let arguments = node.arguments();
    let block = node.block();
    group(array(&[build_index(
        Some(&receiver),
        &separate_docs(
            &[arguments.build(), block.build()],
            array(&[string(COMMA), line()]),
        ),
    )]))
}
