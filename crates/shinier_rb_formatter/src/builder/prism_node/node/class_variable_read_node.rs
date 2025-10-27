use crate::BuildPrismNode;
use crate::document::Document;

use ruby_prism::ClassVariableReadNode;

use crate::BuildContext;

pub fn build_node(node: Option<&ClassVariableReadNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let name = node.name();
    name.build(context)
}
