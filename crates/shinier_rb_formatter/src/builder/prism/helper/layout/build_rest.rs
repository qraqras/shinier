use crate::Build;
use crate::BuildContext;
use crate::ListBuild;
use crate::builder::builder::array;
use crate::document::Document;
use crate::helper::layout::separate_docs;
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
