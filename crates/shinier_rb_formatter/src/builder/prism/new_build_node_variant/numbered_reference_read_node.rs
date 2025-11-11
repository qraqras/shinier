// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/numbered_reference_read_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::numbered_reference_read_node::{layout_numbered_reference_read_node, LayoutParamNumberedReferenceReadNode};

fn build_numbered_reference_read_node(node: &NumberedReferenceReadNode<'_>, context: &mut BuildContext) -> Document {
    let number = Document::String(format!("{}{}", DOLLAR, node.number()));
    layout_numbered_reference_read_node(&LayoutParamNumberedReferenceReadNode { number })
}
