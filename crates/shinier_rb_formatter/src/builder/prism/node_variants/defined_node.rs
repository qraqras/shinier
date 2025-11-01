use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, string};
use crate::document::Document;
use crate::keyword::{DEFINED, PARENTHESES};
use ruby_prism::DefinedNode;

impl<'sh> Build for DefinedNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &DefinedNode, context: &mut BuildContext) -> Document {
    let value = node.value();
    group(array(&[
        string(DEFINED),
        string(PARENTHESES.0),
        value.build(context),
        string(PARENTHESES.1),
    ]))
}
