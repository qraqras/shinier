use crate::builder::Buildable;
use crate::builder::pattern::write_pattern::{
    LogicalWriteNodeTrait, WriteNodeTrait, build_logical_write_pattern,
};
use crate::doc::{Doc, text, text_constant};
use crate::keyword::LOGICAL_AND;
use ruby_prism::ConstantAndWriteNode;

pub fn build_node(node: Option<&ConstantAndWriteNode>) -> Doc {
    let node = node.unwrap();
    build_logical_write_pattern(node)
}

impl<'a> LogicalWriteNodeTrait<'a> for ConstantAndWriteNode<'a> {
    fn logical_operator(&self) -> Doc {
        text(LOGICAL_AND)
    }
}
impl<'a> WriteNodeTrait<'a> for ConstantAndWriteNode<'a> {
    fn name(&self) -> Doc {
        text_constant(&self.name())
    }
    fn value(&self) -> Doc {
        self.value().build()
    }
}
