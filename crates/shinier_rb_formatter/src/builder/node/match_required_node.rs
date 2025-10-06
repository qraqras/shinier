use crate::builder::build;
use crate::doc::{Doc, group, indent, line, sequence, text};
use ruby_prism::*;

const MATCH_KEYWORD: &str = "=>";

pub fn build_node(node: &MatchRequiredNode) -> Doc {
    let value = node.value();
    let pattern = node.pattern();
    group(&[
        build(&value),
        text(" "),
        text(MATCH_KEYWORD),
        line(),
        indent(&[build(&pattern)]),
    ])
}
