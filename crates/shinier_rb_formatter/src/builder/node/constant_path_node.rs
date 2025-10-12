use crate::builder::Buildable;
use crate::builder::layout::separate_docs;
use crate::doc::{Doc, sequence, text};
use crate::keyword::DOUBLE_COLON;
use ruby_prism::ConstantPathNode;

pub fn build_node(node: Option<&ConstantPathNode>) -> Doc {
    let node = node.unwrap();
    let parent = node.parent();
    let name = node.name();
    sequence(&separate_docs(
        &[parent.build(), name.build()],
        &text(DOUBLE_COLON),
    ))
}
