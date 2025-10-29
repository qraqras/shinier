use crate::BuildContext;
use crate::Build;
use crate::builder::builder::{none, string};
use crate::document::Document;
use crate::keyword::TRUE;
use ruby_prism::TrueNode;

impl<'sh> Build for Option<&TrueNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&TrueNode>, _context: &mut BuildContext) -> Document {
    match node {
        Some(_) => string(TRUE),
        None => none(),
    }
}
