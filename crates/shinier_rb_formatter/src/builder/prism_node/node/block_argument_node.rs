use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::*;
use crate::keyword::PROC_AND;
use ruby_prism::BlockArgumentNode;

pub fn build_node(node: Option<&BlockArgumentNode>) -> Doc {
    match node {
        Some(node) => {
            let expression = node.expression();
            group(array(&[string(PROC_AND), expression.build()]))
        }
        None => none(),
    }
}
