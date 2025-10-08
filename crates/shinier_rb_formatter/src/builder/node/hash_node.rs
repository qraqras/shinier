use crate::doc::{Doc, group, indent, line, text};
use crate::layout::separate;
use ruby_prism::HashNode;

const OPEN_DELIMITER: &str = "{";
const CLOSE_DELIMITER: &str = "}";
const SEPARATOR: &str = ",";

pub fn build_node(node: &HashNode) -> Doc {
    let elements = node.elements();
    group(&[
        text(OPEN_DELIMITER),
        indent(&[line(), group(&separate(&elements, SEPARATOR))]),
        line(),
        text(CLOSE_DELIMITER),
    ])
}
