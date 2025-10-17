use crate::builder::Buildable;
use crate::doc::{
    Doc, begin_indent, end_indent, group, hardline, indent, line, sequence, softline, space, text,
};
use crate::keyword::{DEF, DOT_OPERATOR, END, PARENTHESES};
use ruby_prism::DefNode;

pub fn build_node(node: Option<&DefNode>) -> Doc {
    let node = node.unwrap();
    let receiver = node.receiver();
    let name = node.name();
    let parameters = node.parameters();
    let body = node.body();
    group(&[
        text(DEF),
        space(),
        receiver.build_with(None, Some(text(DOT_OPERATOR))),
        name.build(),
        group(&[parameters.build_with(
            Some(sequence(&[text(PARENTHESES.0), softline(), begin_indent()])),
            Some(sequence(&[end_indent(), softline(), text(PARENTHESES.1)])),
        )]),
        indent(&[body.build_with(Some(hardline()), None)]),
        hardline(),
        text(END),
    ])
}
