use crate::builder::builder;
use crate::doc::{Doc, hardline, none, sequence};
use ruby_prism::StatementsNode;

pub fn build_node(node: Option<&StatementsNode>) -> Doc {
    match node {
        Some(node) => {
            let mut statements = Vec::new();
            for node in node.body().iter() {
                statements.push(builder::build(&node));
                statements.push(hardline());
            }
            sequence(&statements)
        }
        None => none(),
    }
}
