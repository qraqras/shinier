use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, indent, line, space, string};
use crate::document::Document;
use crate::keyword::LogicalOperator;
use ruby_prism::OrNode;

impl<'sh> Build for OrNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &OrNode, context: &mut BuildContext) -> Document {
    let left = node.left();
    let right = node.right();
    array(&[
        left.build(context),
        space(),
        string(LogicalOperator::Or.as_str()),
        indent(array(&[line(), right.build(context)])),
    ])
}
