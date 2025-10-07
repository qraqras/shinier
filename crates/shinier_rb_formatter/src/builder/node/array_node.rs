use crate::doc::{Doc, group, indent, softline, text};
use crate::layout::separate;
use ruby_prism::*;

const OPEN_DELIMITER: &str = "[";
const CLOSE_DELIMITER: &str = "]";
const SEPARATOR: &str = ",";

pub fn build_node(node: &ArrayNode) -> Doc {
    let separated = separate(&node.elements(), SEPARATOR);
    group(&[
        text(OPEN_DELIMITER),
        indent(&[softline(), group(&separated)]),
        softline(),
        text(CLOSE_DELIMITER),
    ])
}
