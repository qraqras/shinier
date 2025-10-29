use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::YIELD;
use ruby_prism::YieldNode;

impl<'sh> Build for Option<&YieldNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&YieldNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    group(array(&[
        string(YIELD),
        space(),
        node.arguments().as_ref().build(context),
    ]))
}
