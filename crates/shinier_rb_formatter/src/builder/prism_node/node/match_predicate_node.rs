use crate::builder::Buildable;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::IN;
use ruby_prism::MatchPredicateNode;

pub fn build_node(node: Option<&MatchPredicateNode>) -> Document {
    let node = node.unwrap();
    let value = node.value();
    let pattern = node.pattern();
    group(array(&[
        value.build(),
        space(),
        string(IN),
        space(),
        pattern.build(),
    ]))
}
