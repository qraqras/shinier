use crate::doc::{Doc, group, indent, softline, text};
use crate::layout::separate_nodelist;
use ruby_prism::*;

const OPEN_DELIMITER: &str = "[";
const CLOSE_DELIMITER: &str = "]";
const SEPARATOR: &str = ",";

pub fn build_node(node: Option<&ArrayNode>) -> Doc {
    let node = node.unwrap();
    let elements = node.elements();
    group(&[
        text(OPEN_DELIMITER),
        indent(&[softline(), group(&separate_nodelist(&elements, SEPARATOR))]),
        softline(),
        text(CLOSE_DELIMITER),
    ])
}
