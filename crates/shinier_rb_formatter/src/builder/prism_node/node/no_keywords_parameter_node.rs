use crate::document::Document;
use crate::builder::builder::*;
use ruby_prism::*;

pub fn build_node(node: Option<&NoKeywordsParameterNode>) -> Document {
    let node = node.unwrap();
    return string(format!("not implemented: {:?}", std::any::type_name_of_val(node)));
}
