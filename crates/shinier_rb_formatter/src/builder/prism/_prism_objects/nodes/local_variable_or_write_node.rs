use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use crate::helper::build_write::build_logical_write;
use crate::keyword::LogicalOperator;
use ruby_prism::LocalVariableOrWriteNode;

impl<'sh> Build for LocalVariableOrWriteNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &LocalVariableOrWriteNode, context: &mut BuildContext) -> Document {
    let name = node.name();
    let value = node.value();
    build_logical_write(
        name.build(context),
        value.build(context),
        LogicalOperator::Or,
    )
}
