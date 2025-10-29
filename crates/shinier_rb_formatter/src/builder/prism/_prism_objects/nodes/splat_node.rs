use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, none, string};
use crate::document::Document;
use crate::keyword::ASTERISK;
use ruby_prism::SplatNode;

impl<'sh> Build for Option<&SplatNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&SplatNode>, context: &mut BuildContext) -> Document {
    match node {
        Some(node) => {
            let expression = node.expression();
            array(&[string(ASTERISK), expression.build(context)])
        }
        None => none(),
    }
}
