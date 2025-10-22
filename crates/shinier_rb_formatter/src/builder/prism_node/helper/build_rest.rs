use crate::builder::builder::*;
use crate::builder::{Buildable, BuildableList};
use crate::document::Doc;
use crate::helper::separate_docs::separate_docs;
use ruby_prism::{Node, NodeList};

pub fn build_rest(lefts: NodeList, rest: Option<Node>, rights: NodeList, separator: &Doc) -> Doc {
    array(&separate_docs(
        &[
            lefts.build(separator.clone(), array),
            rest.build(),
            rights.build(separator.clone(), array),
        ],
        separator.clone(),
    ))
}
