use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, string};
use crate::document::Document;
use crate::keyword::{DEFINED, PARENTHESES};
use ruby_prism::DefinedNode;

pub fn build_node(node: Option<&DefinedNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let value = node.value();
    group(array(&[
        string(DEFINED),
        string(PARENTHESES.0),
        value.build(context),
        string(PARENTHESES.1),
    ]))
}
