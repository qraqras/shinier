use crate::buildable::Buildable;
use crate::doc::{Doc, group, indent, none, softline, text};
use crate::keyword::{BRACKETS, SAFE_NAVIGATION_OPERATOR};
use ruby_prism::Node;

pub fn build_index(
    receiver_node: Option<&Node>,
    arguments: &[Doc],
    is_safe_navigation: bool,
) -> Doc {
    match receiver_node {
        Some(node) => group(&[
            node.build(),
            text(BRACKETS.0),
            softline(),
            match is_safe_navigation {
                true => text(SAFE_NAVIGATION_OPERATOR),
                false => none(),
            },
            indent(arguments),
            softline(),
            text(BRACKETS.1),
        ]),
        None => none(),
    }
}
