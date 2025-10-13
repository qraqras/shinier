use crate::builder::Buildable;
use crate::builder::pattern::receiver_pattern::build_receiver_pattern;
use crate::doc::{Doc, sequence, text_constant};
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
            build_receiver_pattern(receiver.as_ref(), is_safe_navigation),
            text_constant(&self.read_name()),
        ])
    }
    fn value(&self) -> Doc {
        self.value().build()
    }
}
