use crate::builder::Buildable;
use crate::doc::{Doc, group, indent, line, space, text};
use ruby_prism::*;

const MATCH_KEYWORD: &str = "=>";

pub fn build_node(node: Option<&MatchRequiredNode>) -> Doc {
    let node = node.unwrap();
    let value = node.value();
    let pattern = node.pattern();
    group(&[
        value.build(),
        space(),
        text(MATCH_KEYWORD),
        line(),
        indent(&[pattern.build()]),
    ])
}
