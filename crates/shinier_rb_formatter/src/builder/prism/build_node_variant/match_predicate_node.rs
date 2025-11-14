// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/match_predicate_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::match_predicate_node::{layout_match_predicate_node, LayoutParamMatchPredicateNode};

pub fn build_match_predicate_node(node: &MatchPredicateNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    let pattern = build_node(&node.pattern(), context);
    layout_match_predicate_node(&LayoutParamMatchPredicateNode { value, pattern })
}
