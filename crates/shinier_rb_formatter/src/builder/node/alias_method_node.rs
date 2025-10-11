use crate::builder::build;
use crate::doc::{Doc, group, indent, line, space, text};
use crate::keyword::ALIAS;
use ruby_prism::AliasMethodNode;

pub fn build_node(node: Option<&AliasMethodNode>) -> Doc {
    let node = node.unwrap();
    let old_name = node.old_name();
    let new_name = node.new_name();
    group(&[
        text(ALIAS),
        line(),
        indent(&[build(&old_name), space(), build(&new_name)]),
    ])
}
