// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/source_line_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;


pub fn build_source_line_node(node: &SourceLineNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(LINE.to_string());
    Document::None
}
