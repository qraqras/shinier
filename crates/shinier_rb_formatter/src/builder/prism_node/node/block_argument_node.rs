use crate::builder::Buildable;
use crate::doc::{Doc, group, none, text};
use crate::keyword::PROC_AND;
use ruby_prism::BlockArgumentNode;

pub fn build_node(node: Option<&BlockArgumentNode>) -> Doc {
    match node {
        Some(node) => {
            let expression = node.expression();
            group(&[text(PROC_AND), expression.build()])
        }
        None => none(),
    }
}
