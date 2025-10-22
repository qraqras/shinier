use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::*;
use crate::keyword::{CLASS, END, INHERITES};
use ruby_prism::ClassNode;

pub fn build_node(node: Option<&ClassNode>) -> Doc {
    let node = node.unwrap();
    let constant_path = node.constant_path();
    let super_class = node.superclass();
    let body = node.body();

    let body_doc = match body {
        Some(_) => indent(array(&[hardline(), body.build()])),
        None => none(),
    };

    group(array(&[
        string(CLASS),
        space(),
        constant_path.build(),
        super_class.build_with(
            Some(array(&[space(), string(INHERITES), space()])),
            Some(space()),
        ),
        body_doc,
        hardline(),
        string(END),
    ]))
}
