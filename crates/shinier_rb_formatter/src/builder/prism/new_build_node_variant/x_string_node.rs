// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/x_string_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::_new_build_node::escape;
use crate::builder::prism::new_layout_node_variant::x_string_node::{layout_x_string_node, LayoutParamXStringNode};

fn build_x_string_node(node: &XStringNode<'_>, context: &mut BuildContext) -> Document {
    // TODO
    let escaped = Document::String(escape(node.unescaped()));
    layout_x_string_node(&LayoutParamXStringNode { escaped })
}
