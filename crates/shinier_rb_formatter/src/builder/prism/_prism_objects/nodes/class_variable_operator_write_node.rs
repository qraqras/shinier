use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use crate::helper::build_write::build_operator_write;
use ruby_prism::ClassVariableOperatorWriteNode;

impl<'sh> Build for Option<&ClassVariableOperatorWriteNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(
    node: Option<&ClassVariableOperatorWriteNode>,
    context: &mut BuildContext,
) -> Document {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    let binary_operator = node.binary_operator();
    build_operator_write(
        name.build(context),
        value.build(context),
        binary_operator.build(context),
    )
}
