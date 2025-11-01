use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::IN;
use ruby_prism::MatchPredicateNode;

impl<'sh> Build for MatchPredicateNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &MatchPredicateNode, context: &mut BuildContext) -> Document {
    let value = node.value();
    let pattern = node.pattern();
    group(array(&[
        value.build(context),
        space(),
        string(IN),
        space(),
        pattern.build(context),
    ]))
}
