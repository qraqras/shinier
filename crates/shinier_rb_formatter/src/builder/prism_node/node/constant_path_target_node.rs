use crate::builder::Buildable;
use crate::doc::{Doc, sequence, text};
use crate::keyword::DOUBLE_COLON;
use ruby_prism::ConstantPathTargetNode;

pub fn build_node(node: Option<&ConstantPathTargetNode>) -> Doc {
    let node = node.unwrap();
    let parent = node.parent();
    let name = node.name();
    sequence(&[parent.build(), text(DOUBLE_COLON), name.build()])
}
