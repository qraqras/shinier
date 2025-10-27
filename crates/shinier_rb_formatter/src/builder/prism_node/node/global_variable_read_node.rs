use crate::BuildContext;
use crate::BuildPrismNode;
use crate::document::Document;
use ruby_prism::GlobalVariableReadNode;

pub fn build_node(node: Option<&GlobalVariableReadNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    node.name().build(context)
}
