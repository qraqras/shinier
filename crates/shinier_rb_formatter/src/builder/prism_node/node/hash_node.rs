use crate::builder::BuildableList;
use crate::doc::{Doc, group, indent, line, sequence, text};
use ruby_prism::HashNode;

const OPEN_DELIMITER: &str = "{";
const CLOSE_DELIMITER: &str = "}";
const SEPARATOR: &str = ",";

pub fn build_node(node: Option<&HashNode>) -> Doc {
    let node = node.unwrap();
    let elements = node.elements();
    group(&[
        text(OPEN_DELIMITER),
        indent(&[
            line(),
            elements.build(sequence(&[text(SEPARATOR), line()]), group),
        ]),
        line(),
        text(CLOSE_DELIMITER),
    ])
}
