// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/source_encoding_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::source_encoding_node::{layout_source_encoding_node, LayoutParamSourceEncodingNode};

fn build_source_encoding_node(node: &SourceEncodingNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(ENCODING.to_string());
    layout_source_encoding_node(&LayoutParamSourceEncodingNode { keyword })
}
