use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::{ALIAS_METHOD, COMMA};
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::AliasMethodNode;
use ruby_prism::Comments;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&AliasMethodNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let old_name = node.old_name();
    let new_name = node.new_name();
    group(array(&[
        string(ALIAS_METHOD),
        space(),
        new_name.build(comments),
        string(COMMA),
        space(),
        old_name.build(comments),
    ]))
}
