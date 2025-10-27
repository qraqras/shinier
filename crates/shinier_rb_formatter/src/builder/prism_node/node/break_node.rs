use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, line, string};
use crate::document::Document;
use crate::keyword::BREAK;
use ruby_prism::BreakNode;

pub fn build_node(node: Option<&BreakNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let arguments = node.arguments();
    group(array(&[
        string(BREAK),
        arguments.build_with(context, Some(line()), None),
    ]))
}
