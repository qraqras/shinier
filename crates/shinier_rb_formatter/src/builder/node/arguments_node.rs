use crate::builder::layout::separate_nodelist;
use crate::doc::{Doc, group, none};
use crate::keyword::COMMA;
use ruby_prism::ArgumentsNode;

pub fn build_node(node: Option<&ArgumentsNode>) -> Doc {
    match node {
        Some(node) => {
            let arguments = node.arguments();
            group(&separate_nodelist(&arguments, COMMA))
        }
        None => none(),
    }
}
