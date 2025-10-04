use crate::builder::build;
use crate::doc::{Doc, group, indent, softline, text, text_from_u8};
use ruby_prism::CallNode;

const OPEN_PAREN: &str = "(";
const CLOSE_PAREN: &str = ")";

pub fn build_node(node: &CallNode) -> Doc {
    let name = text_from_u8(node.name().as_slice());
    let receiver = node.receiver();
    let arguments = node.arguments();
    let block = node.block();

    // 変数呼び出しの場合はそのまま名前を返す
    if node.is_variable_call() {
        return name;
    }
    if node.is_attribute_write() {
        let mut g = Vec::new();
        if let Some(receiver) = receiver {
            g.push(build(&receiver));
            g.push(text("."));
        }
        if let Some(arguments) = arguments {
            g.push(name);
            name.g.push(build(&arguments.as_node()));
        }
        return group(g);
    }

    let mut group_vec = Vec::new();
    if let Some(receiver) = node.receiver() {
        let receiver = build(&receiver);
        group_vec.push(receiver);
        if node.is_safe_navigation() {
            group_vec.push(text("&."));
        }
    }

    let name = text_from_u8(node.name().as_slice());
    group_vec.push(name);

    if let Some(arguments) = node.arguments() {
        let arguments = build(&arguments.as_node());
        let a = group(vec![
            text(OPEN_PAREN),
            softline(),
            indent(arguments),
            softline(),
            text(CLOSE_PAREN),
        ]);
        group_vec.push(a);
    }

    if let Some(block) = node.block() {
        let block = build(&block);
        group_vec.push(block);
    }

    group(group_vec)
}
