use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::builder::builder::{array, group, none, string};
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::PROC_AND;
use ruby_prism::BlockArgumentNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(node: Option<&BlockArgumentNode>, comments: &mut Peekable<Comments>, option: Option<&HashMap<&str, bool>>) -> Document {
    match node {
        Some(node) => {
            let expression = node.expression();
            group(array(&[string(PROC_AND), expression.build(comments)]))
        }
        None => none(),
    }
}
