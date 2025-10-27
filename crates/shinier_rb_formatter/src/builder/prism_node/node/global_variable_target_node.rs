use crate::BuildContext;
use crate::BuildPrismNode;
use crate::document::Document;
use ruby_prism::GlobalVariableTargetNode;

pub fn build_node(node: Option<&GlobalVariableTargetNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let name = node.name();
    name.build(context)
}
