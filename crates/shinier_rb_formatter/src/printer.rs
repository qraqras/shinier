use crate::BuildPrismNode;
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
        }
        if messages.len() > 0 {
            panic!("!!!!パースエラー時の処理は未実装です!!!!: {}", messages);
        }

        let doc = parse_result
            .node()
            .build(&mut parse_result.comments().peekable());

        let output = print_doc_to_string(&doc, ());
        (parse_result, output)
    }
}
