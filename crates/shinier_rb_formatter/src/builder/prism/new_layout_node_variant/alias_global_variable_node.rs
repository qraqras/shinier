// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/alias_global_variable_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamAliasGlobalVariableNode {
    pub new_name: Document,
    pub old_name: Document,
}

pub fn layout_alias_global_variable_node(param: &LayoutParamAliasGlobalVariableNode) -> Document {
    Document::None
}
