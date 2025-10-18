use crate::builder::Buildable;
use crate::doc::{Doc, group, hardline, none, sequence};
use ruby_prism::StatementsNode;

pub fn build_node(node: Option<&StatementsNode>) -> Doc {
    match node {
        Some(node) => {
            let mut statements = Vec::new();
            for (i, node) in node.body().iter().enumerate() {
                if i > 0 {
                    statements.push(hardline());
                }
                statements.push(group(&[node.build()]));
            }
            sequence(&statements)
        }
        None => none(),
    }
}
