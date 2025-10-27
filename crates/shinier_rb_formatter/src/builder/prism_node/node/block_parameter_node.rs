use crate::buildable::Buildable;
use crate::builder::builder::{array, group, none, string};
use crate::document::Document;
use crate::keyword::PROC_AND;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::BlockParameterNode;
use ruby_prism::Comments;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&BlockParameterNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    match node {
        Some(node) => {
            let name = node.name();
            match name {
                Some(name) => group(array(&[string(PROC_AND), name.build(comments)])),
                None => none(),
            }
        }
        None => none(),
    }
}
