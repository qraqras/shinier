use crate::doc::{Doc, none, text};
use crate::keyword::FALSE;
use ruby_prism::FalseNode;

pub fn build_node(node: Option<&FalseNode>) -> Doc {
    match node {
        Some(_) => text(FALSE),
        None => none(),
    }
}
