// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/optional_keyword_parameter_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::keyword::*;
use ruby_prism::*;

pub fn build_optional_keyword_parameter_node(
    node: &OptionalKeywordParameterNode<'_>,
    context: &mut BuildContext,
) -> Document {
    let value = build_node(&node.value(), context);
    Document::None
}
