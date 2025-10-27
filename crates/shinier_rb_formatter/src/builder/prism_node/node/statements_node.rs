use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::builder::builder::{array, group, hardline, none};
use crate::document::Document;
use ruby_prism::Comments;
use ruby_prism::StatementsNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(node: Option<&StatementsNode>, comments: &mut Peekable<Comments>, option: Option<&HashMap<&str, bool>>) -> Document {
    match node {
        Some(node) => {
            let body = node.body();
            let mut vec = Vec::new();
            for (i, s) in body.iter().enumerate() {
                if i > 0 {
                    vec.push(hardline());
                }
                vec.push(group(s.build(comments)));
            }
            array(&vec)
        }
        None => none(),
    }
}
