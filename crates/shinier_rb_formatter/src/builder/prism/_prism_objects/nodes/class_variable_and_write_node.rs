use crate::Build;
use crate::BuildContext;
use crate::builder::prism::helper::layout::build_logical_write;
use crate::document::Document;
use crate::keyword::LogicalOperator;
use ruby_prism::ClassVariableAndWriteNode;

impl<'sh> Build for ClassVariableAndWriteNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &ClassVariableAndWriteNode, context: &mut BuildContext) -> Document {
    let name = node.name();
    let value = node.value();
    build_logical_write(
        name.build(context),
        value.build(context),
        LogicalOperator::And,
    )
}
