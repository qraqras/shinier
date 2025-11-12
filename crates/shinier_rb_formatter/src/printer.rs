use crate::Build;
use crate::BuildContext;
use crate::builder::prism::helper::build_blank_lines::LineBreakIndex;
use crate::builder::prism::helper::build_comments::{
    collect_sorted_node_locations, decorate_comment,
};
use crate::renderer::print_doc_to_string;
use ruby_prism::*;
use std::collections::HashMap;

pub struct Printer<'a> {
    source: String,
    parse_result: Option<ParseResult<'a>>,
    map: Option<HashMap<usize, Comment<'a>>>,
    option: (),
}
impl<'a> Printer<'a> {
    pub fn new(source: String, option: ()) -> Self {
        Self {
            source,
            parse_result: None,
            map: None,
            option,
        }
    }
    pub fn print(&self) -> (ParseResult, String) {
        let parse_result = parse(self.source.as_bytes());

        // TODO: パースエラー時はフォーマットを実施しないようにする
        let mut messages = String::new();
        for diagnostic in parse_result.errors() {
            messages.push_str(diagnostic.message());
            messages.push_str(format!("\n{:?}\n", diagnostic.location()).as_str());
            messages.push_str(format!("\n{}", &self.source).as_str());
        }
        if messages.len() > 0 {
            panic!("!!!!パースエラー時の処理は未実装です!!!!: {}", messages);
        }

        let sorted_node_locations = collect_sorted_node_locations(&parse_result.node());
        let line_break_index = LineBreakIndex::new(self.source.as_bytes());
        let mut comment_metadata = HashMap::new();
        for comment in parse_result.comments() {
            let metadata = decorate_comment(&comment, &sorted_node_locations, &line_break_index);
            comment_metadata.insert(metadata.comment_start_offset, metadata);
        }

        let mut context = BuildContext {
            source: self.source.as_bytes(),
            root: &parse_result.node(),
            built_end: 0usize,
            line_break_index: LineBreakIndex::new(self.source.as_bytes()),
            comments: &mut parse_result.comments().peekable(),
            comment_metadata,
            max_blank_lines: 0usize,
            hash_label_style: false,
            percent_literal: false,
        };
        let mut doc = parse_result.node().build(&mut context);

        let output = print_doc_to_string(&mut doc, ());
        (parse_result, output)
    }
}
