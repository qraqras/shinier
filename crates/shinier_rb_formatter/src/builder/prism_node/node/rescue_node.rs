use crate::builder::{Buildable, BuildableList};
use crate::doc::{Doc, fill, hardline, indent, line, none, sequence, space, text};
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
                exceptions.build(sequence(&[text(EXCEPTIONS_SEPARATOR), line()]), fill),
                match reference {
                    Some(r) => sequence(&[space(), text(REFERENCE_ARROW), line(), r.build()]),
                    None => none(),
                },
                indent(&[statements.build_with(Some(hardline()), None)]),
                subsequent.build_with(Some(hardline()), None),
            ])
        }
        None => none(),
    }
}
