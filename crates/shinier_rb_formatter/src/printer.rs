use crate::builder::builder::*;
use crate::doc::*;
use crate::renderer::Renderer;
use ruby_prism::*;

pub struct Printer {
    src: String,
    option: (),
}
impl Printer {
    pub fn new(src: String, option: ()) -> Self {
        Self { src, option }
    }
    pub fn print(&self) {
        println!("----print----");
        let parsed = self.str_to_ast();
        let docs = self.ast_to_doc(&parsed);
        self.doc_to_str(docs);
    }
    fn str_to_ast(&self) -> ParseResult<'_> {
        println!("----str_to_ast----");
        parse(self.src.as_bytes())
    }
    fn ast_to_doc(&self, parsed: &ParseResult) -> Doc {
        println!("----ast_to_doc----");
        build(&parsed.node())
    }
    fn doc_to_str(&self, doc: Doc) {
        println!("----doc_to_str----");
        const COLUMN_MAX: usize = 20;
        const INDENT_UNIT: &str = "  ";
        let mut renderer = Renderer::new(INDENT_UNIT, COLUMN_MAX);
        renderer.render(doc);
        println!("{}", renderer.output);
    }
}
