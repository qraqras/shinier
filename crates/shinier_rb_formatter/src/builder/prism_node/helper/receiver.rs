use crate::buildable::Buildable;
use crate::doc::{Doc, none, sequence, text};
use crate::keyword::{DOT_OPERATOR, SAFE_NAVIGATION_OPERATOR};
use ruby_prism::Node;

pub fn build_receiver(receiver_node: Option<&Node>, is_safe_navigation: bool) -> Doc {
    match receiver_node {
        Some(node) => sequence(&[
            node.build(),
            match is_safe_navigation {
                true => text(SAFE_NAVIGATION_OPERATOR),
                false => text(DOT_OPERATOR),
            },
        ]),
        None => none(),
    }
}
