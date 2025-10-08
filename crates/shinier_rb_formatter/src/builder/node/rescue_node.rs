use crate::builder::node::statements_node;
use crate::builder::{build, build_optional};
use crate::doc::*;
use crate::layout::separate;
use ruby_prism::*;

const RESCUE_KEYWORD: &str = "rescue";
const REFERENCE_ARROW: &str = "=>";

pub fn build_node(node: Option<&RescueNode>) -> Doc {
    match node {
        Some(node) => {
            let exceptions = node.exceptions();
            let reference = node.reference();
            let statements = node.statements();
            let subsequent = node.subsequent();

            sequence(&[
                text(RESCUE_KEYWORD),
                space(),
                fill(&separate(&exceptions, ",")),
                if let Some(reference) = reference {
                    sequence(&[space(), text(REFERENCE_ARROW), line(), build(&reference)])
                } else {
                    none()
                },
                indent(&[hardline(), statements_node::build_node(statements.as_ref())]),
                build_node(subsequent.as_ref()),
            ])
        }
        None => none(),
    }
}
