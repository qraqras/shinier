use crate::builder::Buildable;
use crate::doc::{Doc, hardline, sequence, text};
use crate::indent;
use ruby_prism::ClassNode;

pub fn build_node(node: Option<&ClassNode>) -> Doc {
    let node = node.unwrap();
    let mut seq = Vec::new();
    seq.push(text("class "));
    seq.push(node.name().build());
    if let Some(superclass) = node.superclass() {
        seq.push(text(" < "));
        seq.push(superclass.build());
    }
    seq.push(hardline());
    if let Some(body) = node.body() {
        seq.push(indent(&[body.build(), hardline()]));
    }
    seq.push(text("end"));
    sequence(seq.as_slice())
}
