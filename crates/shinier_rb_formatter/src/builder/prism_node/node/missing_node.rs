use crate::doc::{Doc, text};
use ruby_prism::MissingNode;

pub fn build_node(node: Option<&MissingNode>) -> Doc {
    let node = node.unwrap();
    text(format!(
        "not implemented: {:?}",
        std::any::type_name_of_val(node)
    ))
}
