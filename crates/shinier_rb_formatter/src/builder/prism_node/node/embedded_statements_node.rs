use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, indent, softline, string};
use crate::document::Document;
use crate::keyword::{BRACES, HASH};
use ruby_prism::EmbeddedStatementsNode;

pub fn build_node(node: Option<&EmbeddedStatementsNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let statements = node.statements();
    group(array(&[
        string(HASH),
        string(BRACES.0),
        indent(array(&[softline(), statements.build(context)])),
        softline(),
        string(BRACES.1),
    ]))
}
