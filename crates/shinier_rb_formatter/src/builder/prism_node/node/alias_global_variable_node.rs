use crate::builder::Buildable;
use crate::doc::{Doc, group, indent, line, space, text};
use crate::keyword::ALIAS;
use ruby_prism::AliasGlobalVariableNode;

pub fn build_node(node: Option<&AliasGlobalVariableNode>) -> Doc {
    let node = node.unwrap();
    let old_name = node.old_name();
    let new_name = node.new_name();
    group(&[
        text(ALIAS),
        line(),
        indent(&[old_name.build(), space(), new_name.build()]),
    ])
}
