use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::keyword::CARET;
use ruby_prism::PinnedVariableNode;

pub fn build_node(node: Option<&PinnedVariableNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let variable = node.variable();
    array(&[string(CARET), variable.build(context)])
}
