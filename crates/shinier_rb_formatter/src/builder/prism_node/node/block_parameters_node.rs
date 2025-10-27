use crate::builder::builder::{array, group, none, space, string};
use crate::builder::node::parameters_node;
use crate::document::Document;
use crate::keyword::{COMMA, SEMI_COLON};
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::BlockParametersNode;
use ruby_prism::Comments;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&BlockParametersNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    match node {
        Some(node) => {
            let parameters = node.parameters();
            let locals = node.locals();
            group(array(&[
                parameters.build(comments),
                match locals.iter().next().is_none() {
                    true => none(),
                    false => locals.build_with(
                        &array(&[string(COMMA), space()]),
                        comments,
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
