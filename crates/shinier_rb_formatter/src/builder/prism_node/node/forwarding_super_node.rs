use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::SUPER;
use ruby_prism::ForwardingSuperNode;

pub fn build_node(node: Option<&ForwardingSuperNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let block = node.block();
    group(array(&[
        string(SUPER),
        block.build_with(context, Some(space()), None),
    ]))
}
