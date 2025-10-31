use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::SUPER;
use ruby_prism::ForwardingSuperNode;

impl<'sh> Build for ForwardingSuperNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &ForwardingSuperNode, context: &mut BuildContext) -> Document {
    let block = node.block();
    group(array(&[
        string(SUPER),
        block.build_with(context, Some(space()), None),
    ]))
}
