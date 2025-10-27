use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, none, string};
use crate::document::Document;
use crate::keyword::ASTERISK;
use ruby_prism::SplatNode;

pub fn build_node(node: Option<&SplatNode>, context: &mut BuildContext) -> Document {
    match node {
        Some(node) => {
            let expression = node.expression();
            array(&[string(ASTERISK), expression.build(context)])
        }
        None => none(),
    }
}
