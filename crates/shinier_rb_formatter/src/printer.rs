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
        println!("----print----");
        let parsed = self.str_to_ast();
        let docs = self.ast_to_doc(&parsed);
        self.doc_to_str(docs)
    }
    fn str_to_ast(&self) -> ParseResult<'_> {
        println!("----str_to_ast----");
        parse(self.src.as_bytes())
    }
    fn ast_to_doc(&self, parsed: &ParseResult) -> Document {
        println!("----ast_to_doc----");
        parsed.node().build()
    }
    fn doc_to_str(&self, doc: Document) -> String {
        println!("----doc_to_str----");
        const COLUMN_MAX: usize = 40;
        const INDENT_UNIT: &str = "  ";
        let output = print_doc_to_string(&doc, ());
        println!("{}", output);
        output
    }
}
