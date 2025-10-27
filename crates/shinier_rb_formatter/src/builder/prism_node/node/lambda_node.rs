use crate::BuildPrismNode;
use crate::builder::builder::{array, group, indent, line, none, softline, space, string};
use crate::builder::node::block_parameters_node;
use crate::document::Document;
use crate::keyword::{ARROW, BRACES, PARENTHESES};

use ruby_prism::LambdaNode;

use crate::BuildContext;

pub fn build_node(
    node: Option<&LambdaNode>,
context: &mut BuildContext
) -> Document {
    let node = node.unwrap();
    let parameters = node.parameters();
    let body = node.body();
    group(array(&[
        string(ARROW),
        match parameters {
            Some(parameters) => {
                if let Some(block_params) = parameters.as_block_parameters_node() {
                    if block_parameters_node::has_parameters(Some(&block_params)) {
                        group(array(&[
                            string(PARENTHESES.0),
                            indent(array(&[softline(), parameters.build(context)])),
                            softline(),
                            string(PARENTHESES.1),
                        ]))
                    } else {
                        none()
                    }
                } else {
                    none()
                }
            }
            _ => none(),
        },
        space(),
        group(array(&[
            string(BRACES.0),
            indent(array(&[line(), body.build(context)])),
            line(),
            string(BRACES.1),
        ])),
    ]))
}
