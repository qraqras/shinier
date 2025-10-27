use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::SUPER;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::Comments;
use ruby_prism::ForwardingSuperNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&ForwardingSuperNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let block = node.block();
    group(array(&[
        string(SUPER),
        block.build_with(comments, Some(space()), None),
    ]))
}
