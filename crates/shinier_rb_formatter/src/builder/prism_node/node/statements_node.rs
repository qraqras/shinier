use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, hardline, none};
use crate::document::Document;
use ruby_prism::StatementsNode;

pub fn build_node(node: Option<&StatementsNode>, context: &mut BuildContext) -> Document {
    match node {
        Some(node) => {
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
        None => none(),
    }
}
