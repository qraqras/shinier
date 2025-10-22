use crate::builder::Buildable;
use crate::builder::builder::{array, group, none, string};
use crate::document::Document;
use crate::keyword::PROC_AND;
use ruby_prism::BlockArgumentNode;

pub fn build_node(node: Option<&BlockArgumentNode>) -> Document {
    match node {
        Some(node) => {
            let expression = node.expression();
            group(array(&[string(PROC_AND), expression.build()]))
        }
        None => none(),
    }
}
