use crate::builder::pattern::write_pattern::{
    LogicalWriteNodeTrait, WriteNodeTrait, build_logical_write_pattern,
};
use crate::builder::{build, build_optional};
use crate::doc::{Doc, sequence, text};
use crate::keyword::{DOT_OPERATOR, SAFE_NAVIGATION_OPERATOR};
use crate::text_constant;
use ruby_prism::CallOrWriteNode;

pub fn build_node(node: Option<&CallOrWriteNode>) -> Doc {
    let node = node.unwrap();
    build_logical_write_pattern(node)
}

impl<'a> LogicalWriteNodeTrait<'a> for CallOrWriteNode<'a> {
    fn logical_operator(&self) -> Doc {
        text(Self::OR_OPERATOR)
    }
}
impl<'a> WriteNodeTrait<'a> for CallOrWriteNode<'a> {
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
