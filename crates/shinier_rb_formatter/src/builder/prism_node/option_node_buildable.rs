use crate::buildable::Buildable;
use crate::document::Document;
use ruby_prism::{
    ArgumentsNode, BlockArgumentNode, BlockNode, BlockParameterNode, ElseNode, EnsureNode,
    ParametersNode, RescueNode, StatementsNode,
};

impl<'a> Buildable<'_> for ArgumentsNode<'_> {
    fn build(&self) -> Document {
        self.as_node().build()
    }
}

impl<'a> Buildable<'_> for BlockArgumentNode<'_> {
    fn build(&self) -> Document {
        self.as_node().build()
    }
}

impl<'a> Buildable<'_> for BlockNode<'_> {
    fn build(&self) -> Document {
        self.as_node().build()
    }
}

impl<'a> Buildable<'_> for BlockParameterNode<'_> {
    fn build(&self) -> Document {
        self.as_node().build()
    }
}

impl<'a> Buildable<'_> for ElseNode<'_> {
    fn build(&self) -> Document {
        self.as_node().build()
    }
}

impl<'a> Buildable<'_> for EnsureNode<'_> {
    fn build(&self) -> Document {
        self.as_node().build()
    }
}

impl<'a> Buildable<'_> for ParametersNode<'_> {
    fn build(&self) -> Document {
        self.as_node().build()
    }
}

impl<'a> Buildable<'_> for RescueNode<'_> {
    fn build(&self) -> Document {
        self.as_node().build()
    }
}

impl<'a> Buildable<'_> for StatementsNode<'_> {
    fn build(&self) -> Document {
        self.as_node().build()
    }
}
