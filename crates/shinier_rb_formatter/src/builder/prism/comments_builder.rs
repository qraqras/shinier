use crate::BuildContext;
use crate::builder::builder::{hardline, string};
use crate::builder::prism::build_leading_line_breaks;
use crate::document::Document;

pub fn build_leading_comments(node: &Node, context: &mut BuildContext) -> Option<Document> {
    let mut vec = Vec::new();
    loop {
        let next_comment_start = context.comments.peek().map(|c| c.location().start_offset());
        match next_comment_start {
            Some(start) => {
                if start < self.location().start_offset() {
                    if let Some(breaks) = build_leading_line_breaks(context, start, 2usize) {
                        vec.push(breaks);
                    }
                    let comment = context.comments.next().unwrap();
                    let mut buf = String::new();
                    comment.text().read_to_string(&mut buf).unwrap();
                    vec.push(string(buf));
                    vec.push(hardline());
                } else {
                    break;
                }
            }
            None => break,
        }
    }
    if vec.is_empty() {
        None
    } else {
        Some(Document::Array(vec))
    }
}

pub fn build_trailing_comments(node: &Node, context: &mut BuildContext) -> Option<Document> {
    let mut vec = Vec::new();
    loop {
        // peek で必要なオフセットだけ取り出して借用を解放
        let next_comment_start = context.comments.peek().map(|c| c.location().start_offset());
        match next_comment_start {
            Some(start) => {
                let node_end = self.location().end_offset();
                // node_end から行末（次の '\n' またはソース終端）を探す
                let mut line_end = node_end;
                while line_end < context.source.len() && context.source[line_end] != b'\n' {
                    line_end += 1;
                }

                // コメントが node_end 以降で行末より前なら trailing comment と判断
                if start >= node_end && start < line_end {
                    // ここで peek の借用は終了しているので comments.next() して可変操作できる
                    let comment = context.comments.next().unwrap();
                    let mut buf = String::new();
                    comment.text().read_to_string(&mut buf).unwrap();
                    // ノードの後の同一行なので空白を挟んでコメントを出す
                    vec.push(string(" "));
                    vec.push(string(buf));
                    // trailing comment は同一行のため hardline を追加しない（必要なら追加）
                    // built_end をコメント末尾まで進める
                    context.built_end = comment.location().end_offset().min(context.source.len());
                    // 続けて同じ行に他のコメントがあるか確認（通常 1 個だがループして安全に）
                    continue;
                } else {
                    break;
                }
            }
            None => break,
        }
    }
    if vec.is_empty() {
        None
    } else {
        Some(Document::Array(vec))
    }
}
