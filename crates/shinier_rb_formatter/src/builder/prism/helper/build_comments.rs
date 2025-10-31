use crate::BuildContext;
use crate::builder::builder::{break_parent, hardline, line_suffix, string};
use crate::builder::prism::build_leading_line_breaks;
use crate::document::Document;
use ruby_prism::Node;
use std::io::Read;

pub fn build_leading_comments(node: &Node, context: &mut BuildContext) -> Option<Document> {
    let mut vec = Vec::new();
    loop {
        let next = context.comments.peek().map(|c| c.location().start_offset());
        match next {
            Some(comment_start_offset) => {
                if comment_start_offset < node.location().start_offset() {
                    if let Some(breaks) =
                        build_leading_line_breaks(context, comment_start_offset, 1usize)
                    {
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

pub fn build_owning_comments(node: &Node, context: &mut BuildContext) -> Document {
    let mut vec = Vec::new();
    loop {
        let next = context.comments.peek().map(|c| c.location().start_offset());
        match next {
            Some(comment_start_offset) => {
                if comment_start_offset >= node.location().start_offset()
                    && comment_start_offset < node.location().end_offset()
                {
                    if let Some(breaks) =
                        build_leading_line_breaks(context, comment_start_offset, 1usize)
                    {
                        vec.push(breaks);
                    }
                    let comment = context.comments.next().unwrap();
                    let mut buf = String::new();
                    comment.text().read_to_string(&mut buf).unwrap();
                    vec.push(string(buf));
                    vec.push(break_parent());
                } else {
                    break;
                }
            }
            None => break,
        }
    }
    if vec.is_empty() {
        Document::None
    } else {
        Document::Array(vec)
    }
}

pub fn build_trailing_comments(node: &Node, context: &mut BuildContext) -> Option<Document> {
    let mut vec = Vec::new();
    loop {
        let next = context.comments.peek().map(|c| c.location().start_offset());
        match next {
            Some(comment_start_offset) => {
                let node_end_offset = node.location().end_offset();
                // ノードの行末を調べる
                let mut line_end_offset = node_end_offset;
                while line_end_offset < context.source.len()
                    && context.source[line_end_offset] != b'\n'
                {
                    line_end_offset += 1;
                }
                // ノードの終端から行末までのコメントを処理する
                if comment_start_offset >= node_end_offset && comment_start_offset < line_end_offset
                {
                    let comment = context.comments.next().unwrap();
                    let mut buf = String::from(' ');
                    comment.text().read_to_string(&mut buf).unwrap();
                    vec.push(line_suffix(string(buf)));
                    vec.push(break_parent());
                    context.built_end = comment.location().end_offset().min(context.source.len());
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
