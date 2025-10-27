use crate::builder::builder::{none, string};
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::FALSE;
use ruby_prism::FalseNode;
use std::collections::HashMap;

pub fn build_node(node: Option<&FalseNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    match node {
        Some(_) => string(FALSE),
        None => none(),
    }
}
