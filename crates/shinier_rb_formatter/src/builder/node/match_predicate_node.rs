use crate::builder::build;
use crate::doc::*;
use ruby_prism::MatchPredicateNode;

const IN_KEYWORD: &str = "in";

pub fn build_node(node: &MatchPredicateNode) -> Doc {
    let value = node.value();
    let pattern = node.pattern();
    sequence(&[
        build(&value),
        space(),
        text(IN_KEYWORD),
        space(),
        build(&pattern),
    ])
}
