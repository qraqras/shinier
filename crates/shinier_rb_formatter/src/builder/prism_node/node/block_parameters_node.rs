use crate::buildable::{Buildable, BuildableList};
use crate::builder::builder::{array, group, none, space, string};
use crate::builder::node::parameters_node;
use crate::document::Document;
use crate::keyword::{COMMA, SEMI_COLON};
use ruby_prism::BlockParametersNode;

pub fn build_node(node: Option<&BlockParametersNode>) -> Document {
    match node {
        Some(node) => {
            let parameters = node.parameters();
            let locals = node.locals();
            group(array(&[
                parameters.build(),
                match locals.is_empty() {
                    true => none(),
                    false => locals.build_with(
                        array(&[string(COMMA), space()]),
                        Some(array(&[string(SEMI_COLON), space()])),
                        None,
                    ),
                },
            ]))
        }
        None => none(),
    }
}

pub fn has_parameters(node: Option<&BlockParametersNode>) -> bool {
    if let Some(node) = node {
        if let Some(parameters) = node.parameters() {
            parameters_node::has_parameters(Some(&parameters))
                || node.locals().iter().next().is_some()
        } else {
            false
        }
    } else {
        false
    }
}
