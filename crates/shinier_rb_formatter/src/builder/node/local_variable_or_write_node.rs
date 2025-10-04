use crate::doc::*;
use ruby_prism::*;

pub fn build_node(node: &LocalVariableOrWriteNode) -> Doc {
        return text(format!("not implemented: {:?}", std::any::type_name_of_val(node)));
}
