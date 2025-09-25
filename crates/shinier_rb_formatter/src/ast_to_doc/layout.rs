use crate::doc::{
    Doc, Docs, fill, group, if_break, indent, line, next_group_id, none, sequence, softline, text,
};

pub fn separate(docs: &Docs, separator: &str) -> Docs {
    let len = docs.len();
    let mut parts = Vec::with_capacity(len * 3);
    for (i, doc) in docs.iter().enumerate() {
        parts.push(doc.clone());
        if i < (len - 1) {
            parts.push(text(separator));
            parts.push(line());
        }
    }
    parts
}

pub fn list_layout(
    docs: &Docs,
    open_delimiter: &str,
    close_delimiter: &str,
    separator: &str,
) -> Doc {
    let group_id = next_group_id();
    // elements
    let mut elements = separate(docs, separator);
    elements.push(if_break(group_id, text(","), none()));
    // layout
    let mut group_member = Vec::with_capacity(5);
    group_member.push(text(open_delimiter));
    group_member.push(softline());
    group_member.push(indent(sequence(vec![softline(), fill(elements)])));
    group_member.push(softline());
    group_member.push(text(close_delimiter));
    group(Some(group_id), group_member)
}
