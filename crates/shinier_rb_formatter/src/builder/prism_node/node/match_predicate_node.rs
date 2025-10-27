use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::IN;
use ruby_prism::MatchPredicateNode;
use std::collections::HashMap;

pub fn build_node(node: Option<&MatchPredicateNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    let node = node.unwrap();
    let value = node.value();
    let pattern = node.pattern();
    group(array(&[
        value.build(comments),
        space(),
        string(IN),
        space(),
        pattern.build(comments),
    ]))
}
