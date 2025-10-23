use crate::builder::Buildable;
use crate::builder::builder::{array, group, indent, line, none, string};
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
                    group(parameters.build_with(Some(line()), None)),
                    group(body.build_with(Some(line()), None)),
                ])),
                line(),
                string(BRACES.1),
            ]))
        }
        None => none(),
    }
}
