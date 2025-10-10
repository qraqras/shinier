use crate::builder::builder;
use crate::doc::{Doc, hardline, none, sequence};
use ruby_prism::StatementsNode;

pub fn build_node(node: Option<&StatementsNode>) -> Doc {
    match node {
        Some(node) => {
            let mut statements = Vec::new();
            for (i, node) in node.body().iter().enumerate() {
                if i > 0 {
                    statements.push(hardline());
                }
                statements.push(builder::build(&node));
            }
            sequence(&statements)
        }
        None => none(),
    }
}
