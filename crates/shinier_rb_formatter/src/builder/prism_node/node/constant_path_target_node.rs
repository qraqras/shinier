use crate::BuildPrismNode;
use crate::builder::builder::{array, string};
use crate::document::Document;

use crate::keyword::DOUBLE_COLON;
use ruby_prism::ConstantPathTargetNode;

use crate::BuildContext;

pub fn build_node(node: Option<&ConstantPathTargetNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let parent = node.parent();
    let name = node.name();
    array(&[
        parent.build(context),
        string(DOUBLE_COLON),
        name.build(context),
    ])
}
