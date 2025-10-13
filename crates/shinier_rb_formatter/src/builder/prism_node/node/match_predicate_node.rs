use crate::builder::Buildable;
use crate::doc::*;
use ruby_prism::MatchPredicateNode;

const IN_KEYWORD: &str = "in";

pub fn build_node(node: Option<&MatchPredicateNode>) -> Doc {
    let node = node.unwrap();
    let value = node.value();
    let pattern = node.pattern();
    sequence(&[
        value.build(),
        space(),
        text(IN_KEYWORD),
        space(),
        pattern.build(),
    ])
}
