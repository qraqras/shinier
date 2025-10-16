use crate::builder::{Buildable, BuildableList};
use crate::doc::{Doc, sequence};
use crate::helper::separate_docs::separate_docs;
use ruby_prism::{Node, NodeList};

pub fn build_rest(lefts: NodeList, rest: Option<Node>, rights: NodeList, separator: &Doc) -> Doc {
    sequence(&separate_docs(
        &[
            lefts.build(separator.clone(), sequence),
            rest.build(),
            rights.build(separator.clone(), sequence),
        ],
        separator.clone(),
    ))
}
