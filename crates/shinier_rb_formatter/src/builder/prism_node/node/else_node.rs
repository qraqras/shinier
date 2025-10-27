use crate::BuildPrismNode;
use crate::builder::builder::{array, group, hardline, indent, none, string};
use crate::document::Document;
use crate::keyword::ELSE;
use ruby_prism::Comments;
use ruby_prism::ElseNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&ElseNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    match node {
        Some(node) => {
            let statements = node.statements();
            group(array(&[
                string(ELSE),
                indent(array(&[statements.build_with(
                    comments,
                    Some(hardline()),
                    None,
                )])),
            ]))
        }
        None => none(),
    }
}
