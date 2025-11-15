use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::AliasGlobalVariableNode;

pub fn build_alias_global_variable_node(
    node: &AliasGlobalVariableNode<'_>,
    context: &mut BuildContext,
) -> Document {
    group(array(&[
        build_location(&node.keyword_loc(), context),
        indent(array(&[
            line(),
            build_node(&node.new_name(), context),
            space(),
            build_node(&node.old_name(), context),
        ])),
    ]))
}
