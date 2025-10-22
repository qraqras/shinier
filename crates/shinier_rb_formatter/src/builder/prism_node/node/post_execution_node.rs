use crate::builder::builder::*;
use crate::document::*;
use ruby_prism::*;

pub fn build_node(node: Option<&PostExecutionNode>) -> Doc {
    let node = node.unwrap();
    return string(format!(
        "not implemented: {:?}",
        std::any::type_name_of_val(node)
    ));
}
