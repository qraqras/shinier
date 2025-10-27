use crate::builder::builder::{array, group, line, none, string};
use crate::document::Document;
use crate::helper::separate_docs::separate_docs;
use crate::keyword::COMMA;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::Comments;
use ruby_prism::ParametersNode;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&ParametersNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    match node {
        Some(node) => {
            let requireds = node.requireds();
            let optionals = node.optionals();
            let rest = node.rest();
            let posts = node.posts();
            let keywords = node.keywords();
            let keyword_rest = node.keyword_rest();
            let block = node.block();

            let separator = array(&[string(COMMA), line()]);
            group(array(&separate_docs(
                &[
                    requireds.build(&separator, comments),
                    optionals.build(&separator, comments),
                    rest.build(comments),
                    posts.build(&separator, comments),
                    keywords.build(&separator, comments),
                    keyword_rest.build(comments),
                    block.build(comments),
                ],
                separator,
            )))
        }
        None => none(),
    }
}

pub fn has_parameters(node: Option<&ParametersNode>) -> bool {
    match node {
        Some(node) => {
            node.requireds().iter().next().is_some()
                || node.optionals().iter().next().is_some()
                || node.rest().is_some()
                || node.posts().iter().next().is_some()
                || node.keywords().iter().next().is_some()
                || node.keyword_rest().is_some()
                || node.block().is_some()
        }
        None => false,
    }
}
