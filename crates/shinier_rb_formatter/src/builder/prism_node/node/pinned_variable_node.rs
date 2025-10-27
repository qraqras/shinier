use crate::BuildPrismNode;
use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::keyword::CARET;
use ruby_prism::Comments;
use ruby_prism::PinnedVariableNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&PinnedVariableNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let variable = node.variable();
    array(&[string(CARET), variable.build(comments)])
}
