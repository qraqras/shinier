use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::builder::helper::separate_docs::separate_docs;
use crate::document::Document;
use crate::keyword::DOUBLE_COLON;
use ruby_prism::ConstantPathNode;

pub fn build_node(node: Option<&ConstantPathNode>) -> Document {
    let node = node.unwrap();
    let parent = node.parent();
    let name = node.name();
    array(&separate_docs(
        &[parent.build(), name.build()],
        string(DOUBLE_COLON),
    ))
}
