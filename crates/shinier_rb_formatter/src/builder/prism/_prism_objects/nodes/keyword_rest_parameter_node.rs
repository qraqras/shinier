use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::keyword::SPLAT;
use ruby_prism::KeywordRestParameterNode;

impl<'sh> Build for Option<&KeywordRestParameterNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&KeywordRestParameterNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let name = node.name();
    array(&[string(SPLAT), name.build(context)])
}
