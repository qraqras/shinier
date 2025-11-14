// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/parameters_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::parameters_node::{layout_parameters_node, LayoutParamParametersNode};

pub fn build_parameters_node(node: &ParametersNode<'_>, context: &mut BuildContext) -> Document {
    let mut requireds = Vec::new();
    for node in &node.requireds() {
        requireds.push(build_node(&node, context));
    }
    let mut optionals = Vec::new();
    for node in &node.optionals() {
        optionals.push(build_node(&node, context));
    }
    let rest = match &node.rest() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let mut posts = Vec::new();
    for node in &node.posts() {
        posts.push(build_node(&node, context));
    }
    let mut keywords = Vec::new();
    for node in &node.keywords() {
        keywords.push(build_node(&node, context));
    }
    let keyword = match &node.keyword_rest() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let block = match &node.block() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_parameters_node(&LayoutParamParametersNode { requireds, optionals, rest, posts, keywords, keyword, block })
}
