use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::*;
use crate::keyword::DOUBLE_COLON;
use ruby_prism::ConstantPathTargetNode;

pub fn build_node(node: Option<&ConstantPathTargetNode>) -> Doc {
    let node = node.unwrap();
    let parent = node.parent();
    let name = node.name();
    array(&[parent.build(), string(DOUBLE_COLON), name.build()])
}
