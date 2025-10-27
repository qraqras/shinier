use crate::builder::builder::array;
use crate::document::Document;
use crate::helper::separate_docs::separate_docs;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::{Comments, Node, NodeList};
use std::iter::Peekable;

pub fn build_rest(
    lefts: NodeList,
    rest: Option<Node>,
    rights: NodeList,
    separator: &Document,
    comments: &mut Peekable<Comments>,
) -> Document {
    array(&separate_docs(
        &[
            lefts.build(&separator, comments),
            rest.build(comments),
            rights.build(&separator, comments),
        ],
        separator.clone(),
    ))
}
