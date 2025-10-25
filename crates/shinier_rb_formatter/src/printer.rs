use crate::buildable::Buildable;
use crate::document::*;
use crate::renderer::print_doc_to_string;
use ruby_prism::*;

pub struct Printer {
    src: String,
    option: (),
}
impl Printer {
    pub fn new(src: String, option: ()) -> Self {
        Self { src, option }
    }
    pub fn print(&self) -> String {
        let parsed = self.str_to_ast();
        let docs = self.ast_to_doc(&parsed);
        self.doc_to_str(docs)
    }
    pub fn str_to_ast(&self) -> ParseResult<'_> {
        parse(self.src.as_bytes())
    }
    pub fn ast_to_doc(&self, parsed: &ParseResult) -> Document {
        parsed.node().build()
    }
    pub fn doc_to_str(&self, doc: Document) -> String {
        const COLUMN_MAX: usize = 40;
        const INDENT_UNIT: &str = "  ";
        let output = print_doc_to_string(&doc, ());
        output
    }
}
