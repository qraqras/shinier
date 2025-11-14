// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/source_line_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::source_line_node::{layout_source_line_node, LayoutParamSourceLineNode};

pub fn build_source_line_node(node: &SourceLineNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(LINE.to_string());
    layout_source_line_node(&LayoutParamSourceLineNode { keyword })
}
