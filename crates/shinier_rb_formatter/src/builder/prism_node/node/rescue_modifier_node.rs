use crate::BuildPrismNode;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::RESCUE;
use ruby_prism::Comments;
use ruby_prism::RescueModifierNode;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&RescueModifierNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let expression = node.expression();
    let rescue_expression = node.rescue_expression();
    group(array(&[
        expression.build(comments),
        space(),
        string(RESCUE),
        space(),
        rescue_expression.build(comments),
    ]))
}
