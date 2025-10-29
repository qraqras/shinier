use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, none, string};
use crate::document::Document;
use crate::keyword::{DOT_OPERATOR, SAFE_NAVIGATION_OPERATOR};
use ruby_prism::Node;

pub fn build_receiver(
    receiver_node: Option<&Node>,
    is_safe_navigation: bool,
    context: &mut BuildContext,
) -> Document {
    match receiver_node {
        Some(node) => array(&[
            node.build(context),
            match is_safe_navigation {
                true => string(SAFE_NAVIGATION_OPERATOR),
                false => string(DOT_OPERATOR),
            },
        ]),
        None => none(),
    }
}
