use crate::builder::build;
use crate::doc::{Doc, none, sequence, text};
use crate::keyword::{DOT_OPERATOR, SAFE_NAVIGATION_OPERATOR};
use ruby_prism::Node;

pub fn build_receiver_pattern(node: Option<&Node>, is_safe_navigation: bool) -> Doc {
    match node {
        Some(node) => sequence(&[
            build(node),
            if is_safe_navigation {
                text(SAFE_NAVIGATION_OPERATOR)
            } else {
                text(DOT_OPERATOR)
            },
        ]),
        None => none(),
    }
}
