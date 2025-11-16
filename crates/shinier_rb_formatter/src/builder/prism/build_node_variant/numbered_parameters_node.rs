// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/numbered_parameters_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::keyword::*;
use ruby_prism::*;

pub fn build_numbered_parameters_node(node: &NumberedParametersNode<'_>, context: &mut BuildContext) -> Document {
    let maximum = Document::String(node.maximum().to_string());
    Document::None
}
