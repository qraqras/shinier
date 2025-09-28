use crate::builder::builder;
use crate::doc::{Doc, hardline, sequence};
use ruby_prism::StatementsNode;

pub fn print(node: &StatementsNode) -> Doc {
    let mut statements = Vec::new();
    for node in node.body().iter() {
        statements.push(builder::build(&node));
        statements.push(hardline());
    }
    sequence(statements)
}
