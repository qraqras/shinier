use crate::BuildContext;
use crate::BuildPrismNode;
use crate::document::Document;
use ruby_prism::InstanceVariableReadNode;

pub fn build_node(node: Option<&InstanceVariableReadNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let name = node.name();
    name.build(context)
}
