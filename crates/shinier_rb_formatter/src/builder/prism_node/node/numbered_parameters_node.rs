use crate::BuildContext;
use crate::builder::builder::string;
use crate::document::Document;
use ruby_prism::NumberedParametersNode;

pub fn build_node(node: Option<&NumberedParametersNode>, _context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let maximum = node.maximum();
    string(maximum.to_string())
}
