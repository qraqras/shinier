use crate::builder::prism_node::node::block_parameter_node;
use crate::builder::{Buildable, BuildableList};
use crate::doc::{Doc, group, line, none, sequence, text};
use crate::helper::separate_docs::separate_docs;
use ruby_prism::ParametersNode;

const PARAMETERS_SEPARATOR: &str = ",";

pub fn build_node(node: Option<&ParametersNode>) -> Doc {
    match node {
        Some(node) => {
            let requireds = node.requireds();
            let optionals = node.optionals();
            let rest = node.rest();
            let posts = node.posts();
            let keywords = node.keywords();
            let keyword_rest = node.keyword_rest();
            let block = node.block();

            let separator = sequence(&[text(PARAMETERS_SEPARATOR), line()]);

            group(&separate_docs(
                &[
                    requireds.build(separator.clone(), sequence),
                    optionals.build(separator.clone(), sequence),
                    rest.build(),
                    posts.build(separator.clone(), sequence),
                    keywords.build(separator.clone(), sequence),
                    keyword_rest.build(),
                    block.build(),
                ],
                separator,
            ))
        }
        None => none(),
    }
}
