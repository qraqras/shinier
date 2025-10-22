use crate::builder::builder::*;
use crate::document::Document;
use ruby_prism::*;

pub fn build_node(node: Option<&ImplicitRestNode>) -> Document {
    let node = node.unwrap();
    return string(format!(
        "not implemented: {:?}",
        std::any::type_name_of_val(node)
    ));
}
