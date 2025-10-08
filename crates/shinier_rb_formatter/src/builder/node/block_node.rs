use crate::doc::*;
use ruby_prism::*;

pub fn build_node(node: Option<&BlockNode>) -> Doc {
    let node = node.unwrap();
    return text(format!("not implemented: {:?}", std::any::type_name_of_val(node)));
}
