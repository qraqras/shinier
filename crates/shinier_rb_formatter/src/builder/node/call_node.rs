use crate::builder::build;
use crate::doc::{Doc, group, indent, softline, text};
use crate::utility::constant_id_to_string;
use ruby_prism::CallNode;

const OPEN_PAREN: &str = "(";
const CLOSE_PAREN: &str = ")";

pub fn build_node(node: &CallNode) -> Doc {
    let name = constant_id_to_string(&node.name());
    let receiver = node.receiver();
    let arguments = node.arguments();
    let block = node.block();

    // 変数呼び出しの場合
    if node.is_variable_call() {
        return text(name);
    }

    let mut vec = Vec::new();

    // レシーバーの書き込み
    if let Some(receiver) = &receiver {
        vec.push(build(receiver));
        if node.is_safe_navigation() {
            vec.push(text("&."));
        } else {
            vec.push(text("."));
        }
    }

    // 属性の書き込みの場合
    if node.is_attribute_write() {
        if let Some(arguments) = arguments {
            let (base_name, op) = split_var_and_op(&name);
            vec.push(text(base_name));
            vec.push(text(" "));
            vec.push(text(op.unwrap_or("")));
            vec.push(text(" "));
            vec.push(build(&arguments.as_node()));
        }
        return group(&vec);
    }

    // 可視性の無視の場合
    if node.is_ignore_visibility() {}

    // その他の場合
    vec.push(text(name));
    if let Some(arguments) = arguments {
        vec.push(group(&[
            text(OPEN_PAREN),
            softline(),
            indent(&[build(&arguments.as_node())]),
            softline(),
            text(CLOSE_PAREN),
        ]));
    }
    if let Some(block) = block {
        vec.push(build(&block));
    }
    group(&vec)
}

// 演算子を末尾から検出して (base_name, Option<op>) を返す
fn split_var_and_op(name: &str) -> (&str, Option<&str>) {
    const OPS: &[&str] = &[
        "**=", "<<=", ">>=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "=",
    ];
    for &op in OPS {
        if name.len() >= op.len() && name.ends_with(op) {
            let base = &name[..name.len() - op.len()];
            return (base, Some(op));
        }
    }
    (name, None)
}
