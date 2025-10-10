use crate::builder::layout::separate_nodelist;
use crate::doc::{Doc, group, none};
use ruby_prism::ArgumentsNode;

const SEPARATER: &str = ",";

pub fn build_node(node: Option<&ArgumentsNode>) -> Doc {
    match node {
        Some(node) => {
            let arguments = node.arguments();
            group(&separate_nodelist(&arguments, SEPARATER))
        }
        None => none(),
    }
}
