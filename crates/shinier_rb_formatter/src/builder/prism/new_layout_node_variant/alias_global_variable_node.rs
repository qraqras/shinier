use crate::Document;
use crate::builder::builder::*;

/// Parameters for layout_alias_global_variable_node function.
pub struct LayoutParamAliasGlobalVariableNode {
    pub leading_comments: Option<Document>,
    pub blank_lines: Option<Document>,
    pub alias_keyword: Option<Document>,
    pub trailing_comment_after_alias_keyword: Option<Document>,
    pub owning_comments: Option<Document>,
    pub new_name: Option<Document>,
    pub old_name: Option<Document>,
    pub trailing_comment: Option<Document>,
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
    array_opt(&[
        param.blank_lines,
        param.leading_comments,
        group_opt(array_opt(&[
            param.alias_keyword,
            param.trailing_comment_after_alias_keyword,
            line_opt(),
            param.owning_comments,
            indent_opt(array_opt(&[param.new_name, space_opt(), param.old_name])),
        ])),
        param.trailing_comment,
    ])
}
