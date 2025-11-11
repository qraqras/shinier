// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/source_file_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::source_file_node::{layout_source_file_node, LayoutParamSourceFileNode};

fn build_source_file_node(node: &SourceFileNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(FILE.to_string());
    layout_source_file_node(&LayoutParamSourceFileNode { keyword })
}
