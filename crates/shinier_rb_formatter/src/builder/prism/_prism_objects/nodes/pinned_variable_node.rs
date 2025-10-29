use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::keyword::CARET;
use ruby_prism::PinnedVariableNode;

impl<'sh> Build for Option<&PinnedVariableNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&PinnedVariableNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let variable = node.variable();
    array(&[string(CARET), variable.build(context)])
}
