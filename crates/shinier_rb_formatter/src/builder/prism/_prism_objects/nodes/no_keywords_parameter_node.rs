use crate::BuildContext;
use crate::Build;
use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::keyword::{NIL, SPLAT};
use ruby_prism::NoKeywordsParameterNode;

impl<'sh> Build for Option<&NoKeywordsParameterNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(
    _node: Option<&NoKeywordsParameterNode>,
    _context: &mut BuildContext,
) -> Document {
    array(&[string(SPLAT), string(NIL)])
}
