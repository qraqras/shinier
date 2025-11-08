// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/retry_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::retry_node::{layout_retry_node, LayoutParamRetryNode};

fn build_retry_node(node: &RetryNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(RETRY.to_string());
    layout_retry_node(&LayoutParamRetryNode { keyword })
}
