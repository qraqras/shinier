use crate::builder::layout::separate_nodelist;
use crate::doc::{Doc, group, line, sequence, text};
use ruby_prism::KeywordHashNode;

const SEPARATER: &str = ",";

pub fn build_node(node: Option<&KeywordHashNode>) -> Doc {
    let node = node.unwrap();
    group(&separate_nodelist(
        &node.elements(),
        &sequence(&[text(SEPARATER), line()]),
    ))
}
