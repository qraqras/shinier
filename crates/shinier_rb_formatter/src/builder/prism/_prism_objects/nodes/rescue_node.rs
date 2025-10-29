use crate::Build;
use crate::BuildContext;
use crate::ListBuild;
use crate::builder::builder::{array, group, hardline, indent, line, none, space, string};
use crate::document::Document;
use crate::keyword::{COMMA, RESCUE, ROCKET};
use ruby_prism::RescueNode;

impl<'sh> Build for Option<&RescueNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

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
                indent(
                    statements
                        .as_ref()
                        .build_with(context, Some(hardline()), None),
                ),
                subsequent
                    .as_ref()
                    .build_with(context, Some(hardline()), None),
            ]))
        }
        None => none(),
    }
}
