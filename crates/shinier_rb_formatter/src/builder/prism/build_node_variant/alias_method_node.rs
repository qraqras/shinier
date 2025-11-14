// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/alias_method_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::alias_method_node::{
    LayoutParamAliasMethodNode, layout_alias_method_node,
};
use crate::keyword::*;
use ruby_prism::*;

pub fn build_alias_method_node(node: &AliasMethodNode<'_>, context: &mut BuildContext) -> Document {
    let new_name = build_node(&node.new_name(), context);
    let old_name = build_node(&node.old_name(), context);
    layout_alias_method_node(&LayoutParamAliasMethodNode { new_name, old_name })
}
