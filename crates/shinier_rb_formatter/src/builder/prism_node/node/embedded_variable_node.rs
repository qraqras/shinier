use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::HASH;
use ruby_prism::EmbeddedVariableNode;

pub fn build_node(node: Option<&EmbeddedVariableNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let variable = node.variable();
    variable.build_with(context, Some(string(HASH)), None)
}
