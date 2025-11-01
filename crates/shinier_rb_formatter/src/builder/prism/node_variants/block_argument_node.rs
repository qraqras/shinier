use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, string};
use crate::document::Document;
use crate::keyword::PROC_AND;
use ruby_prism::BlockArgumentNode;

impl<'sh> Build for BlockArgumentNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &BlockArgumentNode, context: &mut BuildContext) -> Document {
    let expression = node.expression();
    group(array(&[string(PROC_AND), expression.build(context)]))
}
