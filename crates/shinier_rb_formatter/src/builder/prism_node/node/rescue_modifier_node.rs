use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::RESCUE;
use ruby_prism::RescueModifierNode;

pub fn build_node(node: Option<&RescueModifierNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let expression = node.expression();
    let rescue_expression = node.rescue_expression();
    group(array(&[
        expression.build(context),
        space(),
        string(RESCUE),
        space(),
        rescue_expression.build(context),
    ]))
}
