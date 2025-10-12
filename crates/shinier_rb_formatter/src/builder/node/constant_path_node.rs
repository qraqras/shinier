use crate::builder::build_optional;
use crate::builder::layout::separate_docs;
use crate::doc::{Doc, none, sequence, text, text_constant};
use crate::keyword::DOUBLE_COLON;
use ruby_prism::ConstantPathNode;

pub fn build_node(node: Option<&ConstantPathNode>) -> Doc {
    let node = node.unwrap();
    let parent = node.parent();
    let name = node.name();
    sequence(&separate_docs(
        &[
            build_optional(parent.as_ref()),
            match name {
                Some(name) => text_constant(&name),
                None => none(),
            },
        ],
        &text(DOUBLE_COLON),
    ))
}
