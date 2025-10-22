use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::Document;
use ruby_prism::StatementsNode;

pub fn build_node(node: Option<&StatementsNode>) -> Document {
    match node {
        Some(node) => {
            let mut statements = Vec::new();
            for (i, node) in node.body().iter().enumerate() {
                if i > 0 {
                    statements.push(hardline());
                }
                statements.push(group(node.build()));
            }
            array(&statements)
        }
        None => none(),
    }
}
