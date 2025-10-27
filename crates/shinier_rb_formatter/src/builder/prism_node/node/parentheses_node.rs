use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, indent, softline, string};
use crate::document::Document;
use crate::keyword::PARENTHESES;
use ruby_prism::ParenthesesNode;

pub fn build_node(node: Option<&ParenthesesNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let body = node.body();
    group(array(&[
        string(PARENTHESES.0),
        indent(array(&[softline(), body.build(context)])),
        softline(),
        string(PARENTHESES.1),
    ]))
}
