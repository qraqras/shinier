use crate::Build;
use crate::BuildContext;
use crate::builder::prism::helper::layout::build_operator_write;
use crate::document::Document;
use ruby_prism::LocalVariableOperatorWriteNode;

impl<'sh> Build for LocalVariableOperatorWriteNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &LocalVariableOperatorWriteNode, context: &mut BuildContext) -> Document {
    let name = node.name();
    let value = node.value();
    let binary_operator = node.binary_operator();
    build_operator_write(
        name.build(context),
        value.build(context),
        binary_operator.build(context),
    )
}
