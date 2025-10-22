use crate::buildable::Buildable;
use crate::builder::builder::*;
use crate::document::*;
use crate::keyword::ELSE;
use ruby_prism::ElseNode;

pub fn build_node(node: Option<&ElseNode>) -> Doc {
    match node {
        Some(node) => {
            let statements = node.statements();
            group(array(&[
                string(ELSE),
                indent(array(&[statements.build_with(Some(hardline()), None)])),
            ]))
        }
        None => none(),
    }
}
