use crate::Build;
use crate::BuildContext;
use crate::ListBuild;
use crate::builder::builder::{array, group, none, space, string};
use crate::builder::prism::nodes::parameters_node;
use crate::document::Document;
use crate::keyword::{COMMA, SEMI_COLON};
use ruby_prism::BlockParametersNode;

impl<'sh> Build for BlockParametersNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &BlockParametersNode, context: &mut BuildContext) -> Document {
    let parameters = node.parameters();
    let locals = node.locals();
    group(array(&[
        parameters.build(context),
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
