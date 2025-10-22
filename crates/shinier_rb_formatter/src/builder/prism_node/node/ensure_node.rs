use crate::buildable::Buildable;
use crate::builder::builder::*;
use crate::document::Document;
use crate::keyword::ENSURE;
use ruby_prism::EnsureNode;

pub fn build_node(node: Option<&EnsureNode>) -> Document {
    match node {
        Some(node) => {
            let statements = node.statements();
            group(array(&[
                string(ENSURE),
                indent(array(&[statements.build_with(Some(hardline()), None)])),
            ]))
        }
        None => none(),
    }
}
