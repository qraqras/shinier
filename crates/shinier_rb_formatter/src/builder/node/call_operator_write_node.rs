use crate::builder::{build, build_optional};
use crate::doc::{Doc, sequence, text, text_constant};
use crate::keyword::{DOT_OPERATOR, SAFE_NAVIGATION_OPERATOR};
use crate::{OperatorWriteNodeTrait, WriteNodeTrait, build_operator_write_pattern};
use ruby_prism::CallOperatorWriteNode;

pub fn build_node(node: Option<&CallOperatorWriteNode>) -> Doc {
    let node = node.unwrap();
    build_operator_write_pattern(node)
}

impl<'a> OperatorWriteNodeTrait<'a> for CallOperatorWriteNode<'a> {
    fn binary_operator(&self) -> Doc {
        text_constant(&self.binary_operator())
    }
}
impl<'a> WriteNodeTrait<'a> for CallOperatorWriteNode<'a> {
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
