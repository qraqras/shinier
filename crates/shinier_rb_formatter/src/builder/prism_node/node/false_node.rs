use crate::document::*;
use crate::builder::builder::*;
use crate::keyword::FALSE;
use ruby_prism::FalseNode;

pub fn build_node(node: Option<&FalseNode>) -> Doc {
    match node {
        Some(_) => string(FALSE),
        None => none(),
    }
}
