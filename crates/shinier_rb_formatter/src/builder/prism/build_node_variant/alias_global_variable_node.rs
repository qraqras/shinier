use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::helper::build_blank_lines::leading_blank_lines;
use crate::builder::prism::helper::build_comments::*;
use crate::builder::prism::layout_node_variant::alias_global_variable_node::LayoutParamAliasGlobalVariableNode;
use crate::builder::prism::layout_node_variant::alias_global_variable_node::layout_alias_global_variable_node;
use crate::keyword::ALIAS;
use ruby_prism::AliasGlobalVariableNode;

pub fn build_alias_global_variable_node(
    node: &AliasGlobalVariableNode<'_>,
    context: &mut BuildContext,
) -> Document {
    let alias_keyword = Some(build_location(&node.keyword_loc(), context));
    let new_name = Some(build_node(&node.new_name(), context));
    let old_name = Some(build_node(&node.old_name(), context));
    layout_alias_global_variable_node(LayoutParamAliasGlobalVariableNode {
        alias_keyword,
        new_name,
        old_name,
    })
}
