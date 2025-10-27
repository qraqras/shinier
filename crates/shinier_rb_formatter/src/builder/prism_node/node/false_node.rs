use crate::BuildContext;
use crate::builder::builder::{none, string};
use crate::document::Document;
use crate::keyword::FALSE;
use ruby_prism::FalseNode;

pub fn build_node(node: Option<&FalseNode>, _context: &mut BuildContext) -> Document {
    match node {
        Some(_) => string(FALSE),
        None => none(),
    }
}
