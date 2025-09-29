use crate::builder::builder;
use crate::doc::{Doc, fill, group, indent, next_group_id, sequence, softline, text};
use crate::layout::separate;
use ruby_prism::*;

const OPEN_DELIMITER: &str = "[";
const CLOSE_DELIMITER: &str = "]";
const SEPARATOR: &str = ",";

pub fn print(node: &ArrayNode) -> Doc {
    let mut elements = Vec::new();
    for node in node.elements().iter() {
        elements.push(builder::build(&node));
    }
    if elements.is_empty() {
        return sequence(vec![text(OPEN_DELIMITER), text(CLOSE_DELIMITER)]);
    }
    // 要素をセパレータで分割
    let separated = separate(&elements, SEPARATOR);
    // グループを作成
    let mut group_member = Vec::with_capacity(4);
    group_member.push(text(OPEN_DELIMITER));
    group_member.push(indent(sequence(vec![softline(), fill(separated)])));
    group_member.push(softline());
    group_member.push(text(CLOSE_DELIMITER));
    group(group_member)
}
