use crate::BuildPrismNode;
use crate::document::Document;

use crate::helper::build_write::build_logical_write;
use crate::keyword::LogicalOperator;
use ruby_prism::ConstantOrWriteNode;

use crate::BuildContext;

pub fn build_node(node: Option<&ConstantOrWriteNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    build_logical_write(
        name.build(context),
        value.build(context),
        LogicalOperator::Or,
    )
}
