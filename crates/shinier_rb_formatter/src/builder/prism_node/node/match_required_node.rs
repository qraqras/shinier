use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::Document;
use ruby_prism::*;

const MATCH_KEYWORD: &str = "=>";

pub fn build_node(node: Option<&MatchRequiredNode>) -> Document {
    let node = node.unwrap();
    let value = node.value();
    let pattern = node.pattern();
    group(array(&[
        value.build(),
        space(),
        string(MATCH_KEYWORD),
        indent(array(&[line(), pattern.build()])),
    ]))
}
