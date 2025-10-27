use crate::builder::builder::{array, group, string};
use crate::document::Document;
use crate::keyword::SPLAT;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::AssocSplatNode;
use ruby_prism::Comments;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&AssocSplatNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let value = node.value();
    group(array(&[string(SPLAT), value.build(comments)]))
}
