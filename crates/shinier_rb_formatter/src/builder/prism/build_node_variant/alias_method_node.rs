use crate::Document;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::alias_method_node::LayoutParamAliasMethodNode;
use crate::builder::prism::layout_node_variant::alias_method_node::layout_alias_method_node;
use ruby_prism::AliasMethodNode;

pub fn build_alias_method_node(node: &AliasMethodNode<'_>, context: &mut BuildContext) -> Document {
    let alias_keyword = build_location(&node.keyword_loc(), context);
    let new_name = build_node(&node.new_name(), context);
    let old_name = build_node(&node.old_name(), context);
    layout_alias_method_node(LayoutParamAliasMethodNode {
        alias_keyword,
        new_name,
        old_name,
    })
}
