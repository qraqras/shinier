use crate::builder::Buildable;
use crate::builder::prism_node::node::statements_node;
use crate::doc::{Doc, fill, hardline, indent, line, none, sequence, space, text};
use crate::layout::separate_nodelist;
use ruby_prism::RescueNode;

const RESCUE_KEYWORD: &str = "rescue";
const EXCEPTIONS_SEPARATOR: &str = ",";
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
                fill(&separate_nodelist(
                    &exceptions,
                    &sequence(&[text(EXCEPTIONS_SEPARATOR), line()]),
                )),
                match reference {
                    Some(r) => sequence(&[space(), text(REFERENCE_ARROW), line(), r.build()]),
                    None => none(),
                },
                indent(&[
                    hardline(),
                    statements_node::build_node(statements.as_ref()),
                    hardline(),
                ]),
                build_node(subsequent.as_ref()),
            ])
        }
        None => none(),
    }
}
