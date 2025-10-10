use crate::builder::build_optional;
use crate::builder::layout::{separate_docs, separate_nodelist};
use crate::builder::node::block_parameter_node;
use crate::doc::*;
use ruby_prism::*;

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
            group(&separate_docs(&[
                sequence(&separate_nodelist(&requireds, PARAMETERS_SEPARATOR)),
                sequence(&separate_nodelist(&optionals, PARAMETERS_SEPARATOR)),
                build_optional(rest.as_ref()),
                sequence(&separate_nodelist(&posts, PARAMETERS_SEPARATOR)),
                sequence(&separate_nodelist(&keywords, PARAMETERS_SEPARATOR)),
                build_optional(keyword_rest.as_ref()),
                block_parameter_node::build_node(block.as_ref()),
            ]))
        }
        None => none(),
    }
}
