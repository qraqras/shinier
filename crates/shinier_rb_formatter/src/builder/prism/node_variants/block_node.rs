use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, indent, line, string};
use crate::document::Document;
use crate::keyword::{BRACES, PIPE};
use ruby_prism::BlockNode;

impl<'sh> Build for BlockNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &BlockNode, context: &mut BuildContext) -> Document {
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
