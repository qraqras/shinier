use crate::builder::builder::{array, group, hardline, indent, line, none, space, string};
use crate::document::Document;
use crate::keyword::{COMMA, RESCUE, ROCKET};
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::Comments;
use ruby_prism::RescueNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&RescueNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    match node {
        Some(node) => {
            let exceptions = node.exceptions();
            let reference = node.reference();
            let statements = node.statements();
            let subsequent = node.subsequent();
            group(array(&[
                group(array(&[
                    string(RESCUE),
                    space(),
                    exceptions.build(&array(&[string(COMMA), line()]), comments),
                    match reference {
                        Some(r) => array(&[space(), string(ROCKET), line(), r.build(comments)]),
                        None => none(),
                    },
                ])),
                indent(statements.build_with(comments, Some(hardline()), None)),
                subsequent.build_with(comments, Some(hardline()), None),
            ]))
        }
        None => none(),
    }
}
