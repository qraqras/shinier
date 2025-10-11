use crate::builder::build_optional;
use crate::doc::{Doc, group, indent, line, none, text};
use crate::keyword::BRACES;
use ruby_prism::BlockNode;

pub fn build_node(node: Option<&BlockNode>) -> Doc {
    match node {
        Some(node) => {
            let parameters = node.parameters();
            let body = node.body();
            group(&[
                text(BRACES.0),
                line(),
                indent(&[
                    build_optional(parameters.as_ref()),
                    line(),
                    build_optional(body.as_ref()),
                ]),
                line(),
                text(BRACES.1),
            ])
        }
        None => none(),
    }
}
