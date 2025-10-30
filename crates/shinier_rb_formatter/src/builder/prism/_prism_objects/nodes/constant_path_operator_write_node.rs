use crate::Build;
use crate::BuildContext;
use crate::builder::prism::helper::layout::build_operator_write;
use crate::document::Document;
use ruby_prism::ConstantPathOperatorWriteNode;

impl<'sh> Build for ConstantPathOperatorWriteNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &ConstantPathOperatorWriteNode, context: &mut BuildContext) -> Document {
    let target = node.target();
    let value = node.value();
    let binary_operator = node.binary_operator();
    build_operator_write(
        target.as_node().build(context),
        value.build(context),
        binary_operator.build(context),
    )
}
