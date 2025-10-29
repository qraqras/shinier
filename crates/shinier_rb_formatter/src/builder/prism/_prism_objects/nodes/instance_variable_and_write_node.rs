use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use crate::helper::build_write::build_logical_write;
use crate::keyword::LogicalOperator;
use ruby_prism::InstanceVariableAndWriteNode;

impl<'sh> Build for Option<&InstanceVariableAndWriteNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(
    node: Option<&InstanceVariableAndWriteNode>,
    context: &mut BuildContext,
) -> Document {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    build_logical_write(
        name.build(context),
        value.build(context),
        LogicalOperator::And,
    )
}
