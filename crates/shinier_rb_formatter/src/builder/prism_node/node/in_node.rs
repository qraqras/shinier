use crate::buildable::Buildable;
use crate::doc::*;
use crate::keyword::IN;
use ruby_prism::InNode;

pub fn build_node(node: Option<&InNode>) -> Doc {
    let node = node.unwrap();
    let pattern = node.pattern();
    let statements = node.statements();

    sequence(&[
        text(IN),
        space(),
        pattern.build(),
        indent(&[statements.build_with(Some(hardline()), None)]),
    ])
}
