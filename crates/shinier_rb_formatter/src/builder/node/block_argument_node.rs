use crate::build_optional;
use crate::doc::{Doc, none, sequence, text};
use crate::keyword::PROC_AND;
use ruby_prism::BlockArgumentNode;

pub fn build_node(node: Option<&BlockArgumentNode>) -> Doc {
    match node {
        Some(node) => {
            let expression = node.expression();
            sequence(&[text(PROC_AND), build_optional(expression.as_ref())])
        }
        None => none(),
    }
}
