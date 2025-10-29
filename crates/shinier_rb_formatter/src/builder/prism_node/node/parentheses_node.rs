use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, indent, softline, string};
use crate::document::Document;
use crate::keyword::PARENTHESES;
use ruby_prism::ParenthesesNode;

impl<'sh> Build for Option<&ParenthesesNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&ParenthesesNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let body = node.body();
    group(array(&[
        string(PARENTHESES.0),
        indent(array(&[softline(), body.build(context)])),
        softline(),
        string(PARENTHESES.1),
    ]))
}
