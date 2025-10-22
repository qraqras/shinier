use crate::builder::Buildable;
use crate::builder::builder::{array, group, hardline, indent, softline, space, string};
use crate::document::Document;
use crate::keyword::{DEF, DOT_OPERATOR, END, PARENTHESES};
use ruby_prism::DefNode;

pub fn build_node(node: Option<&DefNode>) -> Document {
    let node = node.unwrap();
    let receiver = node.receiver();
    let name = node.name();
    let parameters = node.parameters();
    let body = node.body();
    group(array(&[
        string(DEF),
        space(),
        receiver.build_with(None, Some(string(DOT_OPERATOR))),
        name.build(),
        group(indent(parameters.build_with(
            Some(array(&[string(PARENTHESES.0), softline()])),
            Some(array(&[softline(), string(PARENTHESES.1)])),
        ))),
        indent(array(&[body.build_with(Some(hardline()), None)])),
        hardline(),
        string(END),
    ]))
}
