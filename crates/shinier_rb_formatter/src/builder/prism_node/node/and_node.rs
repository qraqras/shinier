use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, fill, space, string};
use crate::document::Document;
use crate::keyword::LogicalOperator;
use ruby_prism::AndNode;

pub fn build_node(node: Option<&AndNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let left = node.left();
    let right = node.right();
    fill(array(&[
        left.build(context),
        space(),
        string(LogicalOperator::And.as_str()),
        space(),
        right.build(context),
    ]))
}
