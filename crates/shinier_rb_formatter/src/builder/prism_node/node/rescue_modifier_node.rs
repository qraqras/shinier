use crate::buildable::Buildable;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::RESCUE;
use ruby_prism::RescueModifierNode;

pub fn build_node(node: Option<&RescueModifierNode>) -> Document {
    let node = node.unwrap();
    let expression = node.expression();
    let rescue_expression = node.rescue_expression();
    group(array(&[
        expression.build(),
        space(),
        string(RESCUE),
        space(),
        rescue_expression.build(),
    ]))
}
