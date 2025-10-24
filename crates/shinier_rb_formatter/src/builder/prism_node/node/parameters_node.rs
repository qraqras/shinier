use crate::builder::builder::{array, group, line, none, string};
use crate::builder::{Buildable, BuildableList};
use crate::document::Document;
use crate::helper::separate_docs::separate_docs;
use crate::keyword::COMMA;
use ruby_prism::ParametersNode;

pub fn build_node(node: Option<&ParametersNode>) -> Document {
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
                    requireds.build(separator.clone()),
                    optionals.build(separator.clone()),
                    rest.build(),
                    posts.build(separator.clone()),
                    keywords.build(separator.clone()),
                    keyword_rest.build(),
                    block.build(),
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
