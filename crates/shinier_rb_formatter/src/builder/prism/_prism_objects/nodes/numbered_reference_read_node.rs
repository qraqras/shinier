use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::keyword::DOLLAR;
use ruby_prism::NumberedReferenceReadNode;

impl<'sh> Build for NumberedReferenceReadNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &NumberedReferenceReadNode, _context: &mut BuildContext) -> Document {
    let number = node.number();
    array(&[string(DOLLAR), string(number.to_string())])
}
