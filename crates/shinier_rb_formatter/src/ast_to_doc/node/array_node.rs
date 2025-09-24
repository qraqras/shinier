use crate::ast_to_doc::printer;
use crate::doc::*;
use crate::layout::{bracketed_layout, list_layout};
use ruby_prism::*;

const OPEN_DELIMITER: &str = "[";
const CLOSE_DELIMITER: &str = "]";
const SEPARATOR: &str = ", ";

pub fn print(node: &ArrayNode) -> Doc {
    let mut elements = vec![];
    for node in node.elements().iter() {
        elements.push(printer::print(&node));
    }
    if node.is_contains_splat() {
        let op = text("*");
        let arr = elements.first().cloned().unwrap();
        let obj = sequence(vec![op, arr]);
        return bracketed_layout(&obj, OPEN_DELIMITER, CLOSE_DELIMITER);
    }
    list_layout(&elements, OPEN_DELIMITER, CLOSE_DELIMITER, SEPARATOR)
}
