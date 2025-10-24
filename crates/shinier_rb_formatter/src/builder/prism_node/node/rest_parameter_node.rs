use crate::builder::Buildable;
use crate::builder::builder::{array, none, string};
use crate::document::Document;
use crate::keyword::ASTERISK;
use ruby_prism::RestParameterNode;

pub fn build_node(node: Option<&RestParameterNode>) -> Document {
    let node = node.unwrap();
    let name = node.name();
    array(&[
        string(ASTERISK),
        match name {
            Some(name) => name.build(),
            None => none(),
        },
    ])
}
