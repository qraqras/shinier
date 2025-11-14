use crate::Document;
use crate::builder::builder::*;

/// Parameters for layout_alias_global_variable_node function.
pub struct LayoutParamAliasGlobalVariableNode {
    pub alias_keyword: Option<Document>,
    pub new_name: Option<Document>,
    pub old_name: Option<Document>,
}

/// Layouts an AliasGlobalVariableNode.
/// ```ruby
/// alias new_name old_name
/// ```
/// ```ruby
/// alias
///   new_name old_name
/// ```
/// ```ruby
/// # leading comments
/// alias # trailing comment after alias
///   # owning comments
///   new_name old_name # trailing comment
/// ```
pub fn layout_alias_global_variable_node(param: LayoutParamAliasGlobalVariableNode) -> Document {
    group(array_opt(&[
        param.alias_keyword,
        indent_opt(array_opt(&[
            line_opt(),
            param.new_name,
            space_opt(),
            param.old_name,
        ])),
    ]))
}
