use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, indent, line, space, string};
use crate::document::Document;
use crate::keyword::LogicalOperator;
use ruby_prism::OrNode;

pub fn build_node(node: Option<&OrNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let left = node.left();
    let right = node.right();
    array(&[
        left.build(context),
        space(),
        string(LogicalOperator::Or.as_str()),
        indent(array(&[line(), right.build(context)])),
    ])
}
