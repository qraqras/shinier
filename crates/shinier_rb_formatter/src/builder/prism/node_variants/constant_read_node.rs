use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use ruby_prism::ConstantReadNode;

impl<'sh> Build for ConstantReadNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &ConstantReadNode, context: &mut BuildContext) -> Document {
    node.name().build(context)
}
