use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, none, string};
use crate::document::Document;
use crate::keyword::PROC_AND;
use ruby_prism::BlockParameterNode;

pub fn build_node(node: Option<&BlockParameterNode>, context: &mut BuildContext) -> Document {
    match node {
        Some(node) => {
            let name = node.name();
            match name {
                Some(name) => group(array(&[string(PROC_AND), name.build(context)])),
                None => none(),
            }
        }
        None => none(),
    }
}
