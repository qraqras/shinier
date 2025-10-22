use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::Document;
use crate::keyword::BRACES;
use ruby_prism::BlockNode;

pub fn build_node(node: Option<&BlockNode>) -> Document {
    match node {
        Some(node) => {
            let parameters = node.parameters();
            let body = node.body();
            group(array(&[
                string(BRACES.0),
                indent(array(&[
                    parameters.build_with(Some(line()), None),
                    body.build_with(Some(line()), None),
                ])),
                line(),
                string(BRACES.1),
            ]))
        }
        None => none(),
    }
}
