use crate::doc::{Doc, text};
use ruby_prism::MissingNode;

pub fn build_node(node: &MissingNode) -> Doc {
    text(format!(
        "not implemented: {:?}",
        std::any::type_name_of_val(node)
    ))
}
