use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::keyword::IMAGINARY;
use ruby_prism::ImaginaryNode;

impl<'sh> Build for Option<&ImaginaryNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&ImaginaryNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let numeric = node.numeric();
    array(&[numeric.build(context), string(IMAGINARY)])
}
