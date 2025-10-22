use crate::document::*;
use crate::builder::builder::*;
use crate::keyword::TRUE;
use ruby_prism::TrueNode;

pub fn build_node(node: Option<&TrueNode>) -> Doc {
    match node {
        Some(_) => string(TRUE),
        None => none(),
    }
}
