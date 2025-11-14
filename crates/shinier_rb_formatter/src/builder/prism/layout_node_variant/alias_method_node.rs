use crate::Document;
use crate::builder::builder::*;

pub struct LayoutParamAliasMethodNode {
    pub alias_keyword: Document,
    pub new_name: Document,
    pub old_name: Document,
}

pub fn layout_alias_method_node(param: LayoutParamAliasMethodNode) -> Document {
    group(array(&[
        param.alias_keyword,
        indent(array(&[line(), param.new_name, space(), param.old_name])),
    ]))
}
