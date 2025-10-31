use crate::Build;
use crate::BuildContext;
use crate::builder::prism::helper::layout::build_write;
use crate::document::Document;
use ruby_prism::ConstantPathWriteNode;

impl<'sh> Build for ConstantPathWriteNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &ConstantPathWriteNode, context: &mut BuildContext) -> Document {
    let target = node.target();
    let value = node.value();
    build_write(target.as_node().build(context), value.build(context))
}
