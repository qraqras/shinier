use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::BuildContext;
use crate::builder::prism::helper::keyword_trailing_comments;
use crate::builder::prism::helper::leading_blank_lines;
use crate::builder::prism::helper::leading_comments;
use crate::builder::prism::helper::owning_comments;
use crate::builder::prism::helper::trailing_comments;
use crate::builder::prism::new_layout_node_variant::alias_global_variable_node::LayoutParamAliasGlobalVariableNode;
use crate::builder::prism::new_layout_node_variant::alias_global_variable_node::layout_alias_global_variable_node;
use crate::keyword::ALIAS;
use ruby_prism::AliasGlobalVariableNode;

pub fn build_alias_global_variable_node(
    node: &AliasGlobalVariableNode<'_>,
    context: &mut BuildContext,
) -> Document {
    let leading_comments = leading_comments(&node.as_node(), context);
    let blank_lines = leading_blank_lines(&node.as_node(), context);
    let alias_keyword = Some(string(ALIAS));
    let trailing_comment_after_alias_keyword =
        keyword_trailing_comments(node.keyword_loc().end_offset(), context);
    let owning_comments = owning_comments(&node.as_node(), context);
    let new_name = Some(build_node(&node.new_name(), context));
    let old_name = Some(build_node(&node.old_name(), context));
    let trailing_comment = trailing_comments(&node.as_node(), context);
    layout_alias_global_variable_node(LayoutParamAliasGlobalVariableNode {
        leading_comments,
        blank_lines,
        alias_keyword,
        trailing_comment_after_alias_keyword,
        owning_comments,
        new_name,
        old_name,
        trailing_comment,
    })
}
