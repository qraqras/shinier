use crate::builder::build;
use crate::doc::{Doc, group, indent, line, space, text};
use ruby_prism::AliasMethodNode;

const ALIAS_KEYWORD: &str = "alias";

pub fn build_node(node: &AliasMethodNode) -> Doc {
    let old_name = node.old_name();
    let new_name = node.new_name();
    group(&[
        text(ALIAS_KEYWORD),
        line(),
        indent(&[build(&old_name), space(), build(&new_name)]),
    ])
}
