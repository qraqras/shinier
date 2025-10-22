use crate::builder::Buildable;
use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::keyword::DOUBLE_COLON;
use ruby_prism::ConstantPathTargetNode;

pub fn build_node(node: Option<&ConstantPathTargetNode>) -> Document {
    let node = node.unwrap();
    let parent = node.parent();
    let name = node.name();
    array(&[parent.build(), string(DOUBLE_COLON), name.build()])
}
