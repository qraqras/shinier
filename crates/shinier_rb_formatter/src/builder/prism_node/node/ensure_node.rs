use crate::BuildPrismNode;
use crate::builder::builder::{array, group, hardline, indent, none, string};
use crate::document::Document;
use crate::keyword::ENSURE;
use ruby_prism::Comments;
use ruby_prism::EnsureNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&EnsureNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    match node {
        Some(node) => {
            let statements = node.statements();
            group(array(&[
                string(ENSURE),
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
