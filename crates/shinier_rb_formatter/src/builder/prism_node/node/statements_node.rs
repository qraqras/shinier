use crate::builder::Buildable;
use crate::builder::builder::{array, group, hardline, none};
use crate::document::Document;
use ruby_prism::StatementsNode;

pub fn build_node(node: Option<&StatementsNode>) -> Document {
    match node {
        Some(node) => {
            let body = node.body();
            let mut vec = Vec::new();
            for (i, s) in body.iter().enumerate() {
                if i > 0 {
                    vec.push(hardline());
                }
                vec.push(group(s.build()));
            }
            array(&vec)
        }
        None => none(),
    }
}
