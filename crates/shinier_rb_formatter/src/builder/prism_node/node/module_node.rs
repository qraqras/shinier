use crate::buildable::Buildable;
use crate::builder::builder::{array, group, hardline, indent, line, space, string};
use crate::document::Document;
use crate::keyword::{END, MODULE};
use ruby_prism::ModuleNode;

pub fn build_node(node: Option<&ModuleNode>) -> Document {
    let node = node.unwrap();
    let constant_path = node.constant_path();
    let body = node.body();
    group(array(&[
        string(MODULE),
        space(),
        constant_path.build(),
        indent(body.build_with(Some(hardline()), None)),
        line(),
        string(END),
    ]))
}
