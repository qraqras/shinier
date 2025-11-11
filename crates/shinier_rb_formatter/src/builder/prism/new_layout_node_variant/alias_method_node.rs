// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/alias_method_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamAliasMethodNode {
    pub new_name: Document,
    pub old_name: Document,
}

pub fn layout_alias_method_node(param: &LayoutParamAliasMethodNode) -> Document {
    Document::None
}
