use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::keyword::{DOUBLE_QUOTE, SINGLE_QUOTE};
use ruby_prism::StringNode;

impl<'sh> Build for StringNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &StringNode, _context: &mut BuildContext) -> Document {
    let unescaped = node.unescaped();

    let mut should_use_double_quotes = false;
    // 一重引用符が文字列に含まれている場合は二重引用符を使用する
    if unescaped.contains(&b'\x27') {
        should_use_double_quotes = true;
    }
    // エスケープ文字が含まれている場合は二重引用符を使用する
    for escape_char in [
        b'\x09', // tab
        b'\x0b', // vertical tab
        b'\x0a', // newline
        b'\x0d', // carriage return
        b'\x0c', // form feed
        b'\x08', // backspace
        b'\x07', // bell
        b'\x1b', // escape
    ] {
        if unescaped.contains(&escape_char) {
            should_use_double_quotes = true;
            break;
        }
    }

    let (quote, escaped) = if should_use_double_quotes {
        (DOUBLE_QUOTE, escape_double_quote(&unescaped))
    } else {
        (SINGLE_QUOTE, escape_single_quote(&unescaped))
    };

    array(&[string(quote), string(&escaped), string(quote)])
}

fn escape_double_quote(input: &[u8]) -> String {
    let mut result = String::new();
    for &byte in input {
        match byte {
            b'\x5c' => result.push_str("\\\\"), // backslash
            b'\x22' => result.push_str("\\\""), // double quote
            b'\x09' => result.push_str("\\t"),  // tab
            b'\x0b' => result.push_str("\\v"),  // vertical tab
            b'\x0a' => result.push_str("\\n"),  // newline
            b'\x0d' => result.push_str("\\r"),  // carriage return
            b'\x0c' => result.push_str("\\f"),  // form feed
            b'\x08' => result.push_str("\\b"),  // backspace
            b'\x07' => result.push_str("\\a"),  // bell
            b'\x1b' => result.push_str("\\e"),  // escape
            other => result.push(other as char),
        }
    }
    result
}

fn escape_single_quote(input: &[u8]) -> String {
    let mut result = String::new();
    for &byte in input {
        match byte {
            b'\x5c' => result.push_str("\\\\"), // backslash
            b'\x27' => result.push_str("\\'"),  // single quote
            other => result.push(other as char),
        }
    }
    result
}
