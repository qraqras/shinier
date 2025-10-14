use crate::builder::Buildable;
use crate::doc::{Doc, none, sequence, text};
use crate::keyword::PROC_AND;
use ruby_prism::BlockParameterNode;

pub fn build_node(node: Option<&BlockParameterNode>) -> Doc {
    match node {
        Some(node) => {
            let name = node.name();
            match name {
                Some(name) => sequence(&[text(PROC_AND), name.build()]),
                None => none(),
            }
        }
        None => none(),
    }
}
