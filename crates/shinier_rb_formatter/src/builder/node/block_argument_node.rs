use crate::build_optional;
use crate::doc::{Doc, none, sequence, text};
use ruby_prism::BlockArgumentNode;

const BLOCK_ARGUMENT_PREFIX: &str = "&";

pub fn build_node(node: Option<&BlockArgumentNode>) -> Doc {
    match node {
        Some(node) => {
            let expression = node.expression();
            sequence(&[
                text(BLOCK_ARGUMENT_PREFIX),
                build_optional(expression.as_ref()),
            ])
        }
        None => none(),
    }
}
