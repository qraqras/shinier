// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/call_target_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;


pub fn build_call_target_node(node: &CallTargetNode<'_>, context: &mut BuildContext) -> Document {
    let receiver = build_node(&node.receiver(), context);
    Document::None
}
