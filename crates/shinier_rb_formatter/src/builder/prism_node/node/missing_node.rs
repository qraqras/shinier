use crate::document::Document;
use crate::builder::builder::*;
use ruby_prism::MissingNode;

pub fn build_node(node: Option<&MissingNode>) -> Document {
    let node = node.unwrap();
    string(format!(
        "not implemented: {:?}",
        std::any::type_name_of_val(node)
    ))
}
