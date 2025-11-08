// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/missing_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::missing_node::{layout_missing_node, LayoutParamMissingNode};

fn build_missing_node(node: &MissingNode<'_>, context: &mut BuildContext) -> Document {
    layout_missing_node(&LayoutParamMissingNode {  })
}
