use crate::builder::Buildable;
use crate::builder::pattern::receiver_pattern::build_receiver_pattern;
use crate::builder::pattern::write_pattern::{
    LogicalWriteNodeTrait, WriteNodeTrait, build_logical_write_pattern,
};
use crate::doc::{Doc, sequence, text};
use crate::keyword::LOGICAL_OR;
use crate::text_constant;
use ruby_prism::CallOrWriteNode;

pub fn build_node(node: Option<&CallOrWriteNode>) -> Doc {
    let node = node.unwrap();
    build_logical_write_pattern(node)
}

impl<'a> LogicalWriteNodeTrait<'a> for CallOrWriteNode<'a> {
    fn logical_operator(&self) -> Doc {
        text(LOGICAL_OR)
    }
}
impl<'a> WriteNodeTrait<'a> for CallOrWriteNode<'a> {
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
