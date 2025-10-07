use crate::builder::layout::separate;
use crate::doc::{Doc, group};
use ruby_prism::KeywordHashNode;

const SEPARATER: &str = ",";

pub fn build_node(node: &KeywordHashNode) -> Doc {
    group(&separate(&node.elements(), SEPARATER))
}
