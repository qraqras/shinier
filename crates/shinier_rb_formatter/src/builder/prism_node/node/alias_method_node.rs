use crate::buildable::Buildable;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::{ALIAS_METHOD, COMMA};
use ruby_prism::AliasMethodNode;

pub fn build_node(node: Option<&AliasMethodNode>) -> Document {
    let node = node.unwrap();
    let old_name = node.old_name();
    let new_name = node.new_name();
    group(array(&[
        string(ALIAS_METHOD),
        space(),
        new_name.build(),
        string(COMMA),
        space(),
        old_name.build(),
    ]))
}
