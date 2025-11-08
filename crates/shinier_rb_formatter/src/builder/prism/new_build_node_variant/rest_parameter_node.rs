// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/rest_parameter_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::_new_build_node::build_constant_id;
use crate::builder::prism::new_layout_node_variant::rest_parameter_node::{layout_rest_parameter_node, LayoutParamRestParameterNode};

fn build_rest_parameter_node(node: &RestParameterNode<'_>, context: &mut BuildContext) -> Document {
    let name = match &node.name() {
        Some(id) => Some(build_constant_id(id, context)),
        None => None,
    };
    layout_rest_parameter_node(&LayoutParamRestParameterNode { name })
}
