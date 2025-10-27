use crate::BuildPrismNode;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::NEXT;
use ruby_prism::Comments;
use ruby_prism::NextNode;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&NextNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let arguments = node.arguments();
    group(array(&[string(NEXT), space(), arguments.build(comments)]))
}
