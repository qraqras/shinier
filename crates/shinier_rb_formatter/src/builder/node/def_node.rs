use crate::doc::*;
use ruby_prism::*;

pub fn build_node(node: Option<&DefNode>) -> Doc {
    let node = node.unwrap();
    return text(format!("not implemented: {:?}", std::any::type_name_of_val(node)));
}
