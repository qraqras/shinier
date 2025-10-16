use crate::builder::Buildable;
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
                indent(&[
                    parameters.build_with(Some(line()), None),
                    body.build_with(Some(line()), None),
                ]),
                line(),
                text(BRACES.1),
            ])
        }
        None => none(),
    }
}
