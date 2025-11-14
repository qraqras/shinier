// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/it_parameters_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::it_parameters_node::{layout_it_parameters_node, LayoutParamItParametersNode};

pub fn build_it_parameters_node(node: &ItParametersNode<'_>, context: &mut BuildContext) -> Document {
    // TODO
    layout_it_parameters_node(&LayoutParamItParametersNode {  })
}
