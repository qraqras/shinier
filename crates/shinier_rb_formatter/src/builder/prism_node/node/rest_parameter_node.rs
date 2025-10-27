use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::builder::builder::{array, none, string};
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::ASTERISK;
use ruby_prism::RestParameterNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(node: Option<&RestParameterNode>, comments: &mut Peekable<Comments>, option: Option<&HashMap<&str, bool>>) -> Document {
    let node = node.unwrap();
    let name = node.name();
    array(&[
        string(ASTERISK),
        match name {
            Some(name) => name.build(comments),
            None => none(),
        },
    ])
}
