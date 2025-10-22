use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::Document;
use crate::keyword::ALIAS;
use ruby_prism::AliasGlobalVariableNode;

pub fn build_node(node: Option<&AliasGlobalVariableNode>) -> Document {
    let node = node.unwrap();
    let old_name = node.old_name();
    let new_name = node.new_name();
    group(array(&[
        string(ALIAS),
        indent(array(&[
            line(),
            old_name.build(),
            space(),
            new_name.build(),
        ])),
    ]))
}
