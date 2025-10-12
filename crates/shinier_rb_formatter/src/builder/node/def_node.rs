use crate::builder::Buildable;
use crate::builder::node::parameters_node;
use crate::doc::{
    Doc, group, hardline, indent, line, none, sequence, softline, text, text_constant,
};
use ruby_prism::DefNode;

const DEF_KEYWORD: &str = "def";
const END_KEYWORD: &str = "end";
const OPEN_DELIMITER: &str = "(";
const CLOSE_DELIMITER: &str = ")";
const DOT_OPERATOR: &str = ".";

pub fn build_node(node: Option<&DefNode>) -> Doc {
    let node = node.unwrap();
    sequence(&[
        text(DEF_KEYWORD),
        line(),
        build_receiver(&node),
        build_name(&node),
        build_parameters(&node),
        indent(&[hardline(), build_body(&node)]),
        hardline(),
        text(END_KEYWORD),
    ])
}

fn build_receiver(node: &DefNode) -> Doc {
    let receiver = node.receiver();
    match receiver {
        Some(receiver) => sequence(&[receiver.build(), text(DOT_OPERATOR)]),
        None => none(),
    }
}

fn build_name(node: &DefNode) -> Doc {
    let name = node.name();
    text_constant(&name)
}

fn build_parameters(node: &DefNode) -> Doc {
    let parameters = node.parameters();
    match parameters {
        Some(_) => group(&[
            text(OPEN_DELIMITER),
            softline(),
            indent(&[parameters_node::build_node(parameters.as_ref())]),
            softline(),
            text(CLOSE_DELIMITER),
        ]),
        None => none(),
    }
}

fn build_body(node: &DefNode) -> Doc {
    let body = node.body();
    body.build()
}
