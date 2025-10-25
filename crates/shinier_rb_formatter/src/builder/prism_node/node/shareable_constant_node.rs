use crate::buildable::Buildable;
use crate::builder::builder::{array, group, hardline, none, string};
use crate::document::Document;
use crate::keyword::ShareableConstantComment;
use ruby_prism::ShareableConstantNode;

pub fn build_node(node: Option<&ShareableConstantNode>) -> Document {
    let node = node.unwrap();
    let is_literal = node.is_literal();
    let is_experimental_everything = node.is_experimental_everything();
    let is_experimental_copy = node.is_experimental_copy();
    let write = node.write();
    // TODO: コメントの処理
    group(array(&[
        match (is_literal, is_experimental_everything, is_experimental_copy) {
            (true, _, _) => array(&[
                string(ShareableConstantComment::Literal.as_str()),
                hardline(),
            ]),
            (_, true, _) => array(&[
                string(ShareableConstantComment::ExperimentalEverything.as_str()),
                hardline(),
            ]),
            (_, _, true) => array(&[
                string(ShareableConstantComment::ExperimentalCopy.as_str()),
                hardline(),
            ]),
            _ => none(),
        },
        write.build(),
    ]))
}
