use crate::doc::*;
use ruby_prism::*;

pub fn build_node(node: Option<&ElseNode>) -> Doc {
    match node {
        Some(node) => {
            return text(format!(
                "not implemented: {:?}",
                std::any::type_name_of_val(node)
            ));
        }
        None => return none(),
    }
}
