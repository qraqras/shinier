// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/no_keywords_parameter_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;


pub fn build_no_keywords_parameter_node(node: &NoKeywordsParameterNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(format!("{}{}", SPLAT, NIL));
    Document::None
}
