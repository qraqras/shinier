use crate::builder::build;
use crate::doc::{Doc, group, indent, line, text};
use ruby_prism::AliasGlobalVariableNode;

const ALIAS_KEYWORD: &str = "alias";

pub fn build_node(node: &AliasGlobalVariableNode) -> Doc {
    let old_name = node.old_name();
    let new_name = node.new_name();
    group(&[
        text(ALIAS_KEYWORD),
        line(),
        indent(&[build(&old_name), text(" "), build(&new_name)]),
    ])
}
