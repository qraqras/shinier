use crate::builder::Buildable;
use crate::doc::*;
use crate::keyword::{END, IF};
use ruby_prism::*;

pub fn build_node(node: Option<&IfNode>) -> Doc {
    let node = node.unwrap();
    let predicate = node.predicate();
    let statements = node.statements();
    let subsequent = node.subsequent();

    sequence(&[
        text(IF),
        space(),
        predicate.build(),
        space(),
        indent(&[statements.build_with(Some(hardline()), None)]),
        subsequent.build_with(Some(hardline()), None),
        hardline(),
        text(END),
    ])
}
