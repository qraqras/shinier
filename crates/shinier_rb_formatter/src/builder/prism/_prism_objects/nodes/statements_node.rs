use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, hardline};
use crate::document::Document;
use ruby_prism::StatementsNode;

impl<'sh> Build for StatementsNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &StatementsNode, context: &mut BuildContext) -> Document {
    let body = node.body();
    // body.build(context, &none())
    let mut vec = Vec::new();
    for (i, s) in body.iter().enumerate() {
        if i > 0 {
            vec.push(hardline());
        }
        vec.push(group(s.build(context)));
    }
    array(&vec)
}
