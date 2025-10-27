use crate::BuildContext;
use crate::builder::builder::{array, group, hardline, indent, line, none, space, string};
use crate::document::Document;
use crate::keyword::{COMMA, RESCUE, ROCKET};
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::RescueNode;

pub fn build_node(node: Option<&RescueNode>, context: &mut BuildContext) -> Document {
    match node {
        Some(node) => {
            let exceptions = node.exceptions();
            let reference = node.reference();
            let statements = node.statements();
            let subsequent = node.subsequent();
            group(array(&[
                group(array(&[
                    string(RESCUE),
                    space(),
                    exceptions.build(context, &array(&[string(COMMA), line()])),
                    match reference {
                        Some(r) => array(&[space(), string(ROCKET), line(), r.build(context)]),
                        None => none(),
                    },
                ])),
                indent(statements.build_with(context, Some(hardline()), None)),
                subsequent.build_with(context, Some(hardline()), None),
            ]))
        }
        None => none(),
    }
}
