use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::builder::builder::{array, string};
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::DOUBLE_COLON;
use ruby_prism::ConstantPathTargetNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(node: Option<&ConstantPathTargetNode>, comments: &mut Peekable<Comments>, option: Option<&HashMap<&str, bool>>) -> Document {
    let node = node.unwrap();
    let parent = node.parent();
    let name = node.name();
    array(&[parent.build(comments), string(DOUBLE_COLON), name.build(comments)])
}
