// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/string_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::build_node::escape;


pub fn build_string_node(node: &StringNode<'_>, context: &mut BuildContext) -> Document {
    // TODO
    let escaped = Document::String(escape(node.unescaped()));
    Document::None
}
