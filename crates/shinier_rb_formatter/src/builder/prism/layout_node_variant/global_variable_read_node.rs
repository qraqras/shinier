use crate::Document;
use crate::builder::builder::*;

pub struct LayoutParamGlobalVariableReadNode {
    pub name: Document,
}

pub fn layout_global_variable_read_node(param: LayoutParamGlobalVariableReadNode) -> Document {
    group(param.name)
}
