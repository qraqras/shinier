// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/regular_expression_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::build_node::escape;
use crate::keyword::*;
use ruby_prism::*;

pub fn build_regular_expression_node(
    node: &RegularExpressionNode<'_>,
    context: &mut BuildContext,
) -> Document {
    // TODO
    let escaped = Document::String(escape(node.unescaped()));
    Document::None
}
