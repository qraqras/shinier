use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::Document;
use crate::keyword::PROC_AND;
use ruby_prism::BlockParameterNode;

pub fn build_node(node: Option<&BlockParameterNode>) -> Document {
    match node {
        Some(node) => {
            let name = node.name();
            match name {
                Some(name) => group(array(&[string(PROC_AND), name.build()])),
                None => none(),
            }
        }
        None => none(),
    }
}
