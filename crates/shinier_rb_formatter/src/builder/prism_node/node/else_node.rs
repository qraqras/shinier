use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, hardline, indent, none, string};
use crate::document::Document;
use crate::keyword::ELSE;
use ruby_prism::ElseNode;

pub fn build_node(node: Option<&ElseNode>, context: &mut BuildContext) -> Document {
    match node {
        Some(node) => {
            let statements = node.statements();
            group(array(&[
                string(ELSE),
                indent(array(&[statements.build_with(
                    context,
                    Some(hardline()),
                    None,
                )])),
            ]))
        }
        None => none(),
    }
}
