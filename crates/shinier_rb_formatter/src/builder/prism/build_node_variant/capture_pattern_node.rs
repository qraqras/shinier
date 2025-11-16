// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/capture_pattern_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;


pub fn build_capture_pattern_node(node: &CapturePatternNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    let target = build_node(&node.target().as_node(), context);
    Document::None
}
