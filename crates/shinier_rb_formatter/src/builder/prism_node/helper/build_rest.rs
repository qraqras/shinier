use crate::builder::builder::*;
use crate::builder::{Buildable, BuildableList};
use crate::document::Document;
use crate::helper::separate_docs::separate_docs;
use ruby_prism::{Node, NodeList};

pub fn build_rest(lefts: NodeList, rest: Option<Node>, rights: NodeList, separator: &Document) -> Document {
    array(&separate_docs(
        &[
            lefts.build(separator.clone(), array),
            rest.build(),
            rights.build(separator.clone(), array),
        ],
        separator.clone(),
    ))
}
