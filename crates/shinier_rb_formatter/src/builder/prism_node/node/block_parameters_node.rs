use crate::SEMI_COLON;
use crate::buildable::{Buildable, BuildableList};
use crate::builder::builder::*;
use crate::document::Document;
use crate::keyword::{COMMA, PIPE};
use ruby_prism::{BlockParametersNode, NodeList};

pub fn build_node(node: Option<&BlockParametersNode>) -> Document {
    match node {
        Some(node) => {
            let parameters = node.parameters();
            let locals = node.locals();
            group(array(&[
                string(PIPE),
                parameters.build(),
                match is_empty_node_list(&locals) {
                    true => none(),
                    false => locals.build_with(
                        array(&[string(COMMA), space()]),
                        array,
                        Some(array(&[string(SEMI_COLON), space()])),
                        None,
                    ),
                },
                string(PIPE),
            ]))
        }
        None => none(),
    }
}

fn is_empty_node_list(node_list: &NodeList) -> bool {
    node_list.iter().next().is_none()
}
