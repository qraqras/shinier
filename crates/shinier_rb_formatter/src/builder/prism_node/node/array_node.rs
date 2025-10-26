use crate::buildable::{Buildable, BuildableList};
use crate::builder::builder::{array, group, indent, line, none, softline, string};
use crate::document::Document;
use crate::helper::escape::escape_array_element;
use crate::keyword::{
    BRACKETS, COMMA, PERCENT_LOWER_I, PERCENT_LOWER_W, PERCENT_UPPER_I, PERCENT_UPPER_W,
};
use ruby_prism::{ArrayNode, NodeList};

pub fn build_node(node: Option<&ArrayNode>) -> Document {
    let node = node.unwrap();
    let elements = node.elements();

    let mut should_percent_w = true;
    let mut should_percent_i = true;
    let mut should_percent_lower = true;
    // 要素数が0または1の配列は%記法を使用しない
    // TODO: 要素がN個以上の配列で%記法を有効にするオプションを検討する
    if elements.iter().count() <= 1 {
        (should_percent_w, should_percent_i) = (false, false);
    }
    // あらかじめすべての要素を判定して最小限の%記法を選択する
    for element in elements.iter() {
        if should_percent_w {
            (should_percent_w, should_percent_lower) = match (
                element.as_string_node().is_some(),
                element.as_interpolated_string_node().is_some(),
            ) {
                (true, _) => (should_percent_w, should_percent_lower),
                (_, true) => (should_percent_w, false),
                _ => (false, should_percent_lower),
            };
        }
        if should_percent_i {
            (should_percent_i, should_percent_lower) = match (
                element.as_symbol_node().is_some(),
                element.as_interpolated_symbol_node().is_some(),
            ) {
                (true, _) => (should_percent_i, should_percent_lower),
                (_, true) => (should_percent_i, false),
                _ => (false, should_percent_lower),
            };
        }
        if !should_percent_w && !should_percent_i {
            break;
        }
    }
    // 配列の要素をDocumentに変換する
    let elements_array = match (should_percent_w, should_percent_i) {
        (true, _) => array(&build_percent_w_elements(&elements)),
        (_, true) => array(&build_percent_i_elements(&elements)),
        _ => elements.build(array(&[string(COMMA), line()])),
    };
    // 全体をDocumentにする
    group(array(&[
        match (should_percent_w, should_percent_i, should_percent_lower) {
            (true, _, true) => string(PERCENT_LOWER_W),
            (true, _, false) => string(PERCENT_UPPER_W),
            (_, true, true) => string(PERCENT_LOWER_I),
            (_, true, false) => string(PERCENT_UPPER_I),
            _ => none(),
        },
        string(BRACKETS.0),
        indent(array(&[softline(), elements_array])),
        softline(),
        string(BRACKETS.1),
    ]))
}

/// 配列要素が文字列または式展開を含む文字列の場合のDocumentを生成する
fn build_percent_w_elements(elements: &NodeList) -> Vec<Document> {
    let mut documents = Vec::new();
    for (i, element) in elements.iter().enumerate() {
        if i > 0 {
            documents.push(line());
        }
        if let Some(string_node) = element.as_string_node() {
            documents.push(string(escape_array_element(string_node.unescaped())));
        }
        if let Some(interpolated_string_node) = element.as_interpolated_string_node() {
            let mut parts = Vec::new();
            for part in interpolated_string_node.parts().iter() {
                if let Some(string_node) = part.as_string_node() {
                    parts.push(string(escape_array_element(string_node.unescaped())));
                } else {
                    parts.push(part.build());
                }
            }
            documents.push(array(&parts));
        }
    }
    documents
}

/// 配列要素がシンボルまたは式展開を含むシンボルの場合のDocumentを生成する
fn build_percent_i_elements(elements: &NodeList) -> Vec<Document> {
    let mut documents = Vec::new();
    for (i, element) in elements.iter().enumerate() {
        if i > 0 {
            documents.push(line());
        }
        if let Some(symbol_node) = element.as_symbol_node() {
            documents.push(string(escape_array_element(symbol_node.unescaped())));
        }
        if let Some(interpolated_symbol_node) = element.as_interpolated_symbol_node() {
            let mut parts = Vec::new();
            for part in interpolated_symbol_node.parts().iter() {
                if let Some(string_node) = part.as_string_node() {
                    parts.push(string(escape_array_element(string_node.unescaped())));
                } else {
                    parts.push(part.build());
                }
            }
            documents.push(array(&parts));
        }
    }
    documents
}
