use crate::BuildPrismNode;
use crate::document::Document;

use ruby_prism::ClassVariableTargetNode;

use crate::BuildContext;

pub fn build_node(node: Option<&ClassVariableTargetNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let name = node.name();
    name.build(context)
}
