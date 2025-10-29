use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::NEXT;
use ruby_prism::NextNode;

impl<'sh> Build for Option<&NextNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&NextNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let arguments = node.arguments();
    group(array(&[
        string(NEXT),
        space(),
        arguments.as_ref().build(context),
    ]))
}
