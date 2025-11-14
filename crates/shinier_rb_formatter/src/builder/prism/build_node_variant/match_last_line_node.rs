// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/match_last_line_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::build_node::escape;
use crate::builder::prism::BuildContext;
use crate::builder::prism::layout_node_variant::match_last_line_node::{
    LayoutParamMatchLastLineNode, layout_match_last_line_node,
};
use crate::keyword::*;
use ruby_prism::*;

pub fn build_match_last_line_node(
    node: &MatchLastLineNode<'_>,
    context: &mut BuildContext,
) -> Document {
    let escaped = Document::String(escape(node.unescaped()));
    layout_match_last_line_node(&LayoutParamMatchLastLineNode { escaped })
}
