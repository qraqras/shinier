// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/symbol_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::_new_build_node::escape;
use crate::builder::prism::new_layout_node_variant::symbol_node::{layout_symbol_node, LayoutParamSymbolNode};

fn build_symbol_node(node: &SymbolNode<'_>, context: &mut BuildContext) -> Document {
    let escaped = Document::String(escape(node.unescaped()));
    layout_symbol_node(&LayoutParamSymbolNode { escaped })
}
