use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, indent, line, none, string};
use crate::document::Document;
use crate::keyword::{BRACES, PIPE};
use ruby_prism::BlockNode;

pub fn build_node(node: Option<&BlockNode>, context: &mut BuildContext) -> Document {
    match node {
        Some(node) => {
            let parameters = node.parameters();
            let body = node.body();
            group(array(&[
                string(BRACES.0),
                indent(array(&[
                    group(parameters.build_with(
                        context,
                        Some(array(&[line(), string(PIPE)])),
                        Some(string(PIPE)),
                    )),
                    body.build_with(context, Some(line()), None),
                ])),
                line(),
                string(BRACES.1),
            ]))
        }
        None => none(),
    }
}
