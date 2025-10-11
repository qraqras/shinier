use crate::doc::{Doc, none, sequence, text, text_constant};
use crate::keyword::PROC_AND;
use ruby_prism::BlockParameterNode;

pub fn build_node(node: Option<&BlockParameterNode>) -> Doc {
    match node {
        Some(node) => {
            let name = node.name();
            match name {
                Some(name) => sequence(&[text(PROC_AND), text_constant(&name)]),
                None => none(),
            }
        }
        None => none(),
    }
}
