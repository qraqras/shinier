use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::keyword::DOUBLE_QUOTE;
use ruby_prism::StringNode;

pub fn build_node(node: Option<&StringNode>) -> Document {
    let node = node.unwrap();
    let unescaped = node.unescaped();
    array(&[
        string(DOUBLE_QUOTE),
        string(escape(unescaped)),
        string(DOUBLE_QUOTE),
    ])
}

fn escape(input: &[u8]) -> String {
    let mut result = String::new();
    let mut i = 0;
    while i < input.len() {
        match input[i] {
            b'\n' => {
                result.push_str("\\n");
                i += 1;
            }
            b'\t' => {
                result.push_str("\\t");
                i += 1;
            }
            b'\\' => {
                result.push_str("\\\\");
                i += 1;
            }
            b'"' => {
                result.push_str("\\\"");
                i += 1;
            }
            other => {
                result.push(other as char);
                i += 1;
            }
        }
    }
    result
}
