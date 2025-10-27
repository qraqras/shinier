use crate::builder::BuildContext;
use crate::builder::builder::array;
use crate::document::Document;
use crate::helper::separate_docs::separate_docs;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::{Node, NodeList};

pub fn build_rest(
    lefts: NodeList,
    rest: Option<Node>,
    rights: NodeList,
    separator: &Document,
    context: &mut BuildContext,
) -> Document {
    array(&separate_docs(
        &[
            lefts.build(context, &separator),
            rest.build(context),
            rights.build(context, &separator),
        ],
        separator.clone(),
    ))
}
