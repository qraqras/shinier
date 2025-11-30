use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::ClassVariableOperatorWriteNode;

/// Builds ClassVariableOperatorWriteNode.
pub fn build_class_variable_operator_write_node(
    node: &ClassVariableOperatorWriteNode<'_>,
    ctx: &mut BuildContext,
) -> Option<Document> {
    let name_loc = node.name_loc();
    let binary_operator_loc = node.binary_operator_loc();
    let value = node.value();

    group(array(&[
        build_location(name_loc, ctx),
        space(),
        build_location(binary_operator_loc, ctx),
        indent(array(&[line(), build_node(value, ctx)])),
    ]))
}
