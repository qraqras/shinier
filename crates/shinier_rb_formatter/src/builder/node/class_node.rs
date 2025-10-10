use crate::builder::builder::build;
use crate::doc::{Doc, hardline, sequence, text, text_from_u8};
use crate::indent;
use ruby_prism::ClassNode;

pub fn build_node(node: Option<&ClassNode>) -> Doc {
    let node = node.unwrap();
    let mut seq = Vec::new();
    seq.push(text("class "));
    seq.push(text_from_u8(node.name().as_slice()));
    if let Some(superclass) = node.superclass() {
        seq.push(text(" < "));
        seq.push(build(&superclass));
    }
    seq.push(hardline());
    if let Some(body) = node.body() {
        seq.push(indent(&[build(&body), hardline()]));
    }
    seq.push(text("end"));
    sequence(seq.as_slice())
}
