// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/module_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamModuleNode {
    pub constant_path: Document,
    pub body: Vec<Document>,
}

pub fn layout_module_node(param: &LayoutParamModuleNode) -> Document {
    Document::None
}
