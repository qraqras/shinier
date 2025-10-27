use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, hardline, indent, none, string};
use crate::document::Document;
use crate::keyword::ENSURE;
use ruby_prism::EnsureNode;

pub fn build_node(node: Option<&EnsureNode>, context: &mut BuildContext) -> Document {
    match node {
        Some(node) => {
            let statements = node.statements();
            group(array(&[
                string(ENSURE),
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
