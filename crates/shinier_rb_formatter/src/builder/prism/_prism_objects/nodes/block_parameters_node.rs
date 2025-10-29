use crate::Build;
use crate::BuildContext;
use crate::ListBuild;
use crate::builder::builder::{array, group, none, space, string};
use crate::builder::node::parameters_node;
use crate::document::Document;
use crate::keyword::{COMMA, SEMI_COLON};
use ruby_prism::BlockParametersNode;

impl<'sh> Build for Option<&BlockParametersNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&BlockParametersNode>, context: &mut BuildContext) -> Document {
    match node {
        Some(node) => {
            let parameters = node.parameters();
            let locals = node.locals();
            group(array(&[
                parameters.as_ref().build(context),
                match locals.iter().next().is_none() {
                    true => none(),
                    false => locals.build_with(
                        context,
                        &array(&[string(COMMA), space()]),
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
