use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::ALIAS;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::AliasGlobalVariableNode;
use ruby_prism::Comments;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&AliasGlobalVariableNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let old_name = node.old_name();
    let new_name = node.new_name();
    group(array(&[
        string(ALIAS),
        space(),
        old_name.build(comments),
        space(),
        new_name.build(comments),
    ]))
}
