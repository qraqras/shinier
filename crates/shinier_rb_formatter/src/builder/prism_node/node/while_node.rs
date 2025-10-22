use crate::document::*;
use crate::builder::builder::*;
use ruby_prism::*;

pub fn build_node(node: Option<&WhileNode>) -> Doc {
    let node = node.unwrap();
    return string(format!("not implemented: {:?}", std::any::type_name_of_val(node)));
}
