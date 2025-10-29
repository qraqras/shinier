use crate::BuildContext;
use crate::Build;
use crate::builder::builder::{none, string};
use crate::document::Document;
use crate::keyword::FALSE;
use ruby_prism::FalseNode;

impl<'sh> Build for Option<&FalseNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&FalseNode>, _context: &mut BuildContext) -> Document {
    match node {
        Some(_) => string(FALSE),
        None => none(),
    }
}
