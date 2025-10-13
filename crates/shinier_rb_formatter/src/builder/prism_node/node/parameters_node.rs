use crate::builder::Buildable;
use crate::builder::layout::{separate_docs, separate_nodelist};
use crate::builder::prism_node::node::block_parameter_node;
use crate::doc::{Doc, group, line, none, sequence, text};
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
                    sequence(&separate_nodelist(&requireds, &separator)),
                    sequence(&separate_nodelist(&optionals, &separator)),
                    rest.build(),
                    sequence(&separate_nodelist(&posts, &separator)),
                    sequence(&separate_nodelist(&keywords, &separator)),
                    keyword_rest.build(),
                    block_parameter_node::build_node(block.as_ref()),
                ],
                &separator,
            ))
        }
        None => none(),
    }
}
