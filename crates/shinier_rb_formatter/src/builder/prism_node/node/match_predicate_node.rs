use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::IN;
use ruby_prism::MatchPredicateNode;

pub fn build_node(node: Option<&MatchPredicateNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
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
