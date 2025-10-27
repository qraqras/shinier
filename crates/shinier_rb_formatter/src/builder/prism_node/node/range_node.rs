use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, string};
use crate::document::Document;
use crate::keyword::{DOUBLE_DOT, TRIPLE_DOT};
use ruby_prism::RangeNode;

pub fn build_node(node: Option<&RangeNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let is_exclude_end = node.is_exclude_end();
    let left = node.left();
    let right = node.right();
    group(array(&[
        left.build(context),
        match is_exclude_end {
            true => string(TRIPLE_DOT),
            false => string(DOUBLE_DOT),
        },
        right.build(context),
    ]))
}
