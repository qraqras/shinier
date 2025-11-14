// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/module_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::module_node::{layout_module_node, LayoutParamModuleNode};

pub fn build_module_node(node: &ModuleNode<'_>, context: &mut BuildContext) -> Document {
    let constant_path = build_node(&node.constant_path(), context);
    let mut body = Vec::new();
    if let Some(node) = &node.body() {
        body.push(build_node(&node, context));
    }
    layout_module_node(&LayoutParamModuleNode { constant_path, body })
}
