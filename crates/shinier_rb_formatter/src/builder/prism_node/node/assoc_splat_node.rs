use crate::builder::builder::{array, group, string};
use crate::document::Document;
use crate::keyword::SPLAT;
use crate::{BuildContext, BuildPrismNode};
use ruby_prism::AssocSplatNode;

pub fn build_node(node: Option<&AssocSplatNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let value = node.value();
    group(array(&[string(SPLAT), value.build(context)]))
}
