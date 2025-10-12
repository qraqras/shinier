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
                line(),
                indent(&[parameters.build(), line(), body.build()]),
                line(),
                text(BRACES.1),
            ])
        }
        None => none(),
    }
}
