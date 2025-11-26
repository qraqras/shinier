// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/it_local_variable_read_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::keyword::*;
use ruby_prism::*;

pub fn build_it_local_variable_read_node(
    node: &ItLocalVariableReadNode<'_>,
    ctx: &mut BuildContext,
) -> Option<Document> {
    None
}
