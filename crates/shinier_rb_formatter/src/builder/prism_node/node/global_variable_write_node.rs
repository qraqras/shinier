use crate::BuildPrismNode;
use crate::document::Document;
use crate::helper::build_write::build_write;

use ruby_prism::GlobalVariableWriteNode;

use crate::BuildContext;

pub fn build_node(
    node: Option<&GlobalVariableWriteNode>,
context: &mut BuildContext
) -> Document {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    build_write(name.build(context), value.build(context))
}
