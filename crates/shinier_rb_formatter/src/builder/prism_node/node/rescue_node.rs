use crate::builder::builder::*;
use crate::builder::{Buildable, BuildableList};
use crate::document::Document;
use ruby_prism::RescueNode;

const RESCUE_KEYWORD: &str = "rescue";
const EXCEPTIONS_SEPARATOR: &str = ",";
const REFERENCE_ARROW: &str = "=>";

pub fn build_node(node: Option<&RescueNode>) -> Document {
    match node {
        Some(node) => {
            let exceptions = node.exceptions();
            let reference = node.reference();
            let statements = node.statements();
            let subsequent = node.subsequent();

            group(array(&[
                group(array(&[
                    string(RESCUE_KEYWORD),
                    space(),
                    exceptions.build(array(&[string(EXCEPTIONS_SEPARATOR), line()])),
                    match reference {
                        Some(r) => array(&[space(), string(REFERENCE_ARROW), line(), r.build()]),
                        None => none(),
                    },
                ])),
                indent(statements.build_with(Some(hardline()), None)),
                subsequent.build_with(Some(hardline()), None),
            ]))
        }
        None => none(),
    }
}
