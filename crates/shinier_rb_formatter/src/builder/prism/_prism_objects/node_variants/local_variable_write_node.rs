use crate::Build;
use crate::BuildContext;
use crate::builder::prism::helper::layout::build_write;
use crate::document::Document;
use ruby_prism::LocalVariableWriteNode;

impl<'sh> Build for LocalVariableWriteNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &LocalVariableWriteNode, context: &mut BuildContext) -> Document {
    let name = node.name();
    let value = node.value();
    build_write(name.build(context), value.build(context))
}
