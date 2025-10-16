use crate::doc::{Doc, none, text};
use crate::keyword::TRUE;
use ruby_prism::TrueNode;

pub fn build_node(node: Option<&TrueNode>) -> Doc {
    match node {
        Some(_) => text(TRUE),
        None => none(),
    }
}
