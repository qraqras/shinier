use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, string};
use crate::builder::helper::separate_docs::separate_docs;
use crate::document::Document;
use crate::keyword::DOUBLE_COLON;
use ruby_prism::ConstantPathNode;

impl<'sh> Build for ConstantPathNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &ConstantPathNode, context: &mut BuildContext) -> Document {
    let parent = node.parent();
    let name = node.name();
    array(&separate_docs(
        &[parent.build(context), name.build(context)],
        string(DOUBLE_COLON),
    ))
}
