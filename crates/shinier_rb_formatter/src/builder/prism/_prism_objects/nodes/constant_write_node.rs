use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use crate::helper::build_write::build_write;
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
