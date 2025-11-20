// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/constant_path_and_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::keyword::*;
use ruby_prism::*;

pub fn build_constant_path_and_write_node(node: &ConstantPathAndWriteNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let target = build_node(&node.target().as_node(), ctx);
    let value = build_node(&node.value(), ctx);
    None
}
