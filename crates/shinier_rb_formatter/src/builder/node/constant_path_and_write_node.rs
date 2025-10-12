use crate::builder::build;
use crate::builder::pattern::write_pattern::{
    LogicalWriteNodeTrait, WriteNodeTrait, build_logical_write_pattern,
};
use crate::doc::{Doc, text};
use crate::keyword::LOGICAL_AND;
use ruby_prism::ConstantPathAndWriteNode;

pub fn build_node(node: Option<&ConstantPathAndWriteNode>) -> Doc {
    let node = node.unwrap();
    build_logical_write_pattern(node)
}

impl<'a> LogicalWriteNodeTrait<'a> for ConstantPathAndWriteNode<'a> {
    fn logical_operator(&self) -> Doc {
        text(LOGICAL_AND)
    }
}
impl<'a> WriteNodeTrait<'a> for ConstantPathAndWriteNode<'a> {
    fn name(&self) -> Doc {
        build(&self.target().as_node())
    }
    fn value(&self) -> Doc {
        build(&self.value())
    }
}
