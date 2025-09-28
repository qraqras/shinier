use crate::builder::builder;
use crate::doc::{
    Doc, fill, group, if_break, indent, next_group_id, none, sequence, softline, text,
};
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
    // 空のとき
    if elements.is_empty() {
        return sequence(vec![text(OPEN_DELIMITER), text(CLOSE_DELIMITER)]);
    }
    // グループIDを取得
    let group_id = next_group_id();
    // 要素をセパレータで分割して末尾のカンマをつける
    let mut separated = separate(&elements, SEPARATOR);
    separated.push(if_break(group_id, text(SEPARATOR), none()));
    // グループを作成
    let mut group_member = Vec::with_capacity(4);
    group_member.push(text(OPEN_DELIMITER));
    group_member.push(indent(sequence(vec![softline(), fill(separated)])));
    group_member.push(softline());
    group_member.push(text(CLOSE_DELIMITER));
    group(Some(group_id), group_member)
}
