use crate::ast_to_doc::printer;
use crate::doc::{Doc, hardline, sequence};
use ruby_prism::StatementsNode;

pub fn print(node: &StatementsNode) -> Doc {
    let mut statements = vec![];
    for node in node.body().iter() {
        statements.push(printer::print(&node));
        statements.push(hardline());
    }
    sequence(statements)
}
