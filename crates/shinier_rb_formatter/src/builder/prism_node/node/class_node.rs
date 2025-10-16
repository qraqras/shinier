use crate::builder::Buildable;
use crate::doc::{Doc, hardline, indent, sequence, space, text};
use crate::keyword::{CLASS, END, INHERITES};
use ruby_prism::ClassNode;

pub fn build_node(node: Option<&ClassNode>) -> Doc {
    let node = node.unwrap();
    let constant_path = node.constant_path();
    let super_class = node.superclass();
    let body = node.body();
    sequence(&[
        text(CLASS),
        space(),
        constant_path.build(),
        super_class.build_with(
            Some(sequence(&[space(), text(INHERITES), space()])),
            Some(space()),
        ),
        indent(&[body.build_with(Some(hardline()), None)]),
        hardline(),
        text(END),
    ])
}
