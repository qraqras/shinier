use crate::BuildPrismNode;
use crate::builder::builder::group;
use crate::document::Document;
use ruby_prism::Comments;
use ruby_prism::MatchWriteNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&MatchWriteNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let call = node.call();
    group(call.as_node().build(comments))
}
