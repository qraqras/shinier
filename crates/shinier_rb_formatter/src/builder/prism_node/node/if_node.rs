use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::Document;
use crate::keyword::{END, IF};
use ruby_prism::*;

pub fn build_node(node: Option<&IfNode>) -> Document {
    let node = node.unwrap();
    let predicate = node.predicate();
    let statements = node.statements();
    let subsequent = node.subsequent();

    array(&[
        string(IF),
        space(),
        predicate.build(),
        space(),
        indent(array(&[statements.build_with(Some(hardline()), None)])),
        subsequent.build_with(Some(hardline()), None),
        hardline(),
        string(END),
    ])
}
