use crate::doc::{Doc, fill, group, indent, sequence, softline, text};
use crate::layout::separate;
use ruby_prism::*;

const OPEN_DELIMITER: &str = "[";
const CLOSE_DELIMITER: &str = "]";
const SEPARATOR: &str = ",";

pub fn build_node(node: &ArrayNode) -> Doc {
    // 要素をセパレータで分割
    let separated = separate(&node.elements(), SEPARATOR);
    // グループを作成
    let mut group_member = Vec::with_capacity(4);
    group_member.push(text(OPEN_DELIMITER));
    group_member.push(indent(sequence(vec![softline(), fill(separated)])));
    group_member.push(softline());
    group_member.push(text(CLOSE_DELIMITER));
    group(group_member)
}
