use crate::builder::pattern::write_pattern::{
    LogicalWriteNodeTrait, WriteNodeTrait, build_logical_write_pattern,
};
use crate::builder::{build, build_optional};
use crate::doc::{Doc, sequence, text, text_constant};
use crate::keyword::{DOT_OPERATOR, SAFE_NAVIGATION_OPERATOR};
use ruby_prism::CallAndWriteNode;

pub fn build_node(node: Option<&CallAndWriteNode>) -> Doc {
    let node = node.unwrap();
    build_logical_write_pattern(node)
}
impl<'a> LogicalWriteNodeTrait<'a> for CallAndWriteNode<'a> {
    fn logical_operator(&self) -> Doc {
        text(Self::AND_OPERATOR)
    }
}
impl<'a> WriteNodeTrait<'a> for CallAndWriteNode<'a> {
    fn name(&self) -> Doc {
        let is_safe_navigation = self.is_safe_navigation();
        let receiver = self.receiver();
        sequence(&[
            build_optional(receiver.as_ref()),
            match is_safe_navigation {
                true => text(SAFE_NAVIGATION_OPERATOR),
                false => text(DOT_OPERATOR),
            },
            text_constant(&self.read_name()),
        ])
    }
    fn value(&self) -> Doc {
        build(&self.value())
    }
}
