use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::ALTERNATION;
use ruby_prism::AlternationPatternNode;

impl<'sh> Build for Option<&AlternationPatternNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&AlternationPatternNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let left = node.left();
    let right = node.right();
    group(array(&[
        left.build(context),
        space(),
        string(ALTERNATION),
        space(),
        right.build(context),
    ]))
}
