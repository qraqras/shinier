use crate::builder::builder::{array, group, hardline, indent, line, softline, space, string};
use crate::document::Document;
use crate::keyword::{DEF, DOT_OPERATOR, END, PARENTHESES};
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::Comments;
use ruby_prism::DefNode;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&DefNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let receiver = node.receiver();
    let name = node.name();
    let parameters = node.parameters();
    let body = node.body();
    group(array(&[
        string(DEF),
        space(),
        receiver.build_with(comments, None, Some(string(DOT_OPERATOR))),
        name.build(comments),
        group(indent(parameters.build_with(
            comments,
            Some(array(&[string(PARENTHESES.0), softline()])),
            Some(array(&[softline(), string(PARENTHESES.1)])),
        ))),
        indent(array(&[body.build_with(comments, Some(hardline()), None)])),
        line(),
        string(END),
    ]))
}
