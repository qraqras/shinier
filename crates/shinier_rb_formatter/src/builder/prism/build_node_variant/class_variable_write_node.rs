use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::ClassVariableWriteNode;

pub fn build_class_variable_write_node(node: &ClassVariableWriteNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let name_loc = node.name_loc();
    let value = node.value();
    let operator_loc = node.operator_loc();

    group(array(&[
        build_location(name_loc, ctx),
        space(),
        build_location(operator_loc, ctx),
        indent(group(array(&[line(), build_node(value, ctx)]))),
    ]))
}
