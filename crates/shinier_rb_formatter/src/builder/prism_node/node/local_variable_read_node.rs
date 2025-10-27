use crate::BuildContext;
use crate::BuildPrismNode;
use crate::document::Document;
use ruby_prism::LocalVariableReadNode;

pub fn build_node(node: Option<&LocalVariableReadNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let name = node.name();
    name.build(context)
}
