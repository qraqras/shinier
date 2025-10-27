use crate::BuildPrismNode;
use crate::builder::builder::{array, string};
use crate::builder::helper::separate_docs::separate_docs;
use crate::document::Document;
use crate::keyword::DOUBLE_COLON;
use ruby_prism::ConstantPathNode;

use crate::BuildContext;

pub fn build_node(node: Option<&ConstantPathNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let parent = node.parent();
    let name = node.name();
    array(&separate_docs(
        &[parent.build(context), name.build(context)],
        string(DOUBLE_COLON),
    ))
}
