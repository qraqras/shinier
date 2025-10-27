use crate::BuildPrismNode;
use crate::builder::builder::{array, none, string};
use crate::document::Document;
use crate::keyword::{DOT_OPERATOR, SAFE_NAVIGATION_OPERATOR};
use ruby_prism::{Comments, Node};
use std::iter::Peekable;

pub fn build_receiver(
    receiver_node: Option<&Node>,
    is_safe_navigation: bool,
    comments: &mut Peekable<Comments>,
) -> Document {
    match receiver_node {
        Some(node) => array(&[
            node.build(comments),
            match is_safe_navigation {
                true => string(SAFE_NAVIGATION_OPERATOR),
                false => string(DOT_OPERATOR),
            },
        ]),
        None => none(),
    }
}
