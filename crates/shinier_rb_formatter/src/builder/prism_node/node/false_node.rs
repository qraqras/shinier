use crate::builder::builder::{none, string};
use crate::document::Document;
use crate::keyword::FALSE;
use ruby_prism::FalseNode;

pub fn build_node(node: Option<&FalseNode>) -> Document {
    match node {
        Some(_) => string(FALSE),
        None => none(),
    }
}
