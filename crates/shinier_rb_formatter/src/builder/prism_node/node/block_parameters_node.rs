use crate::SEMI_COLON;
use crate::buildable::{Buildable, BuildableList};
use crate::doc::{Doc, none, sequence, space, text};
use crate::keyword::{COMMA, PIPE};
use ruby_prism::{BlockParametersNode, NodeList};

pub fn build_node(node: Option<&BlockParametersNode>) -> Doc {
    match node {
        Some(node) => {
            let parameters = node.parameters();
            let locals = node.locals();
            sequence(&[
                text(PIPE),
                parameters.build(),
                match is_empty_node_list(&locals) {
                    true => none(),
                    false => locals.build_with(
                        sequence(&[text(COMMA), space()]),
                        sequence,
                        Some(sequence(&[text(SEMI_COLON), space()])),
                        None,
                    ),
                },
                text(PIPE),
            ])
        }
        None => none(),
    }
}

fn is_empty_node_list(node_list: &NodeList) -> bool {
    node_list.iter().next().is_none()
}
