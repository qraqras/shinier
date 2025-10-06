use crate::builder::build;
use crate::doc::*;
use ruby_prism::*;

const ALTERNATION_OPERATOR: &str = "|";

pub fn build_node(node: &AlternationPatternNode) -> Doc {
    let left = node.left();
    let right = node.right();
    sequence(&[
        build(&left),
        text(" "),
        text(ALTERNATION_OPERATOR),
        line(),
        build(&right),
    ])
}
