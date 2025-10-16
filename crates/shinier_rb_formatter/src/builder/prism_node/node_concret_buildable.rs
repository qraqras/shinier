use crate::buildable::Buildable;
use crate::doc::{Doc, none};
use ruby_prism::{ArgumentsNode, BlockArgumentNode};

impl<'a> Buildable<'_> for Option<ArgumentsNode<'_>> {
    fn build(&self) -> Doc {
        match self {
            Some(arguments_node) => arguments_node.as_node().build(),
            None => none(),
        }
    }
}

impl<'a> Buildable<'_> for Option<BlockArgumentNode<'_>> {
    fn build(&self) -> Doc {
        match self {
            Some(block_node) => block_node.as_node().build(),
            None => none(),
        }
    }
}
