use crate::builder::builder::{array, group, indent, none, softline, string};
use crate::document::Document;
use crate::keyword::BRACKETS;
use crate::{BuildContext, BuildPrismNode};
use ruby_prism::Node;

pub fn build_index(
    receiver_node: Option<&Node>,
    arguments: &[Document],
    context: &mut BuildContext,
) -> Document {
    match receiver_node {
        Some(receiver) => group(array(&[
            receiver.build(context),
            string(BRACKETS.0),
            indent(array(&[softline(), array(arguments)])),
            softline(),
            string(BRACKETS.1),
        ])),
        None => none(),
    }
}
