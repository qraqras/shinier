use crate::Build;
use crate::BuildContext;
use crate::builder::prism::helper::layout::build_write;
use crate::document::Document;
use ruby_prism::ConstantWriteNode;

impl<'sh> Build for ConstantWriteNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &ConstantWriteNode, context: &mut BuildContext) -> Document {
    let name = node.name();
    let value = node.value();
    build_write(name.build(context), value.build(context))
}
