use crate::BuildPrismNode;
use crate::builder::builder::{array, string};
use crate::builder::helper::separate_docs::separate_docs;
use crate::document::Document;
use crate::keyword::DOUBLE_COLON;
use ruby_prism::{Comments, ConstantPathNode};
use std::collections::HashMap;

pub fn build_node(
    node: Option<&ConstantPathNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let parent = node.parent();
    let name = node.name();
    array(&separate_docs(
        &[parent.build(comments), name.build(comments)],
        string(DOUBLE_COLON),
    ))
}
