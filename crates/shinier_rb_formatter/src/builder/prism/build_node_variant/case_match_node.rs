// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/case_match_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::case_match_node::{layout_case_match_node, LayoutParamCaseMatchNode};

pub fn build_case_match_node(node: &CaseMatchNode<'_>, context: &mut BuildContext) -> Document {
    let predicate = match &node.predicate() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let mut conditions = Vec::new();
    for node in &node.conditions() {
        conditions.push(build_node(&node, context));
    }
    let else_clause = match &node.else_clause() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_case_match_node(&LayoutParamCaseMatchNode { predicate, conditions, else_clause })
}
