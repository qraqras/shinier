use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::builder::builder::{array, group, none, string};
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::PROC_AND;
use ruby_prism::BlockArgumentNode;
use std::collections::HashMap;

pub fn build_node(node: Option<&BlockArgumentNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    match node {
        Some(node) => {
            let expression = node.expression();
            group(array(&[string(PROC_AND), expression.build(comments)]))
        }
        None => none(),
    }
}
