use crate::builder::BuildableList;
use crate::builder::builder::*;
use crate::document::*;
use ruby_prism::HashNode;

const OPEN_DELIMITER: &str = "{";
const CLOSE_DELIMITER: &str = "}";
const SEPARATOR: &str = ",";

pub fn build_node(node: Option<&HashNode>) -> Doc {
    let node = node.unwrap();
    let elements = node.elements();
    group(array(&[
        string(OPEN_DELIMITER),
        indent(array(&[
            line(),
            elements.build(array(&[string(SEPARATOR), line()]), array),
        ])),
        line(),
        string(CLOSE_DELIMITER),
    ]))
}
