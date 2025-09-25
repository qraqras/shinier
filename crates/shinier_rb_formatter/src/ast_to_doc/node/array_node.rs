use crate::ast_to_doc::printer;
use crate::doc::Doc;
use crate::layout::list_layout;
use ruby_prism::*;

const OPEN_DELIMITER: &str = "[";
const CLOSE_DELIMITER: &str = "]";
const SEPARATOR: &str = ",";

pub fn print(node: &ArrayNode) -> Doc {
    let mut elements = Vec::new();
    for node in node.elements().iter() {
        elements.push(printer::print(&node));
    }
    list_layout(&elements, OPEN_DELIMITER, CLOSE_DELIMITER, SEPARATOR)
}
