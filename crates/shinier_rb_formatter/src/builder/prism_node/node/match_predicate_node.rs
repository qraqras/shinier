use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::*;
use ruby_prism::MatchPredicateNode;

const IN_KEYWORD: &str = "in";

pub fn build_node(node: Option<&MatchPredicateNode>) -> Doc {
    let node = node.unwrap();
    let value = node.value();
    let pattern = node.pattern();
    array(&[
        value.build(),
        space(),
        string(IN_KEYWORD),
        space(),
        pattern.build(),
    ])
}
