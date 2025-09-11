use crate::doc::*;
use crate::visitor::Visitor;
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
        let parsed = self.src_to_ast();
        let docs = self.ast_to_doc(&parsed);
        self.doc_to_src(docs);
    }
    fn src_to_ast(&self) -> ParseResult<'_> {
        println!("----src_to_ast----");
        parse(self.src.as_bytes())
    }
    fn ast_to_doc(&self, parsed: &ParseResult) -> Docs {
        println!("----ast_to_doc----");
        let mut visitor = Visitor::new();
        visitor.visit(&parsed.node());
        visitor.docs
    }
    fn doc_to_src(&self, docs: Docs) {
        fn print_docs(doc: &dyn Doc) {
            if let Some(children) = doc.children() {
                for child in children {
                    print_docs(child.as_ref());
                }
            } else {
                println!("{:?}", doc.content().unwrap());
            }
        }
        println!("----doc_to_src----");
        for doc in docs {
            print_docs(doc.as_ref());
        }
    }
}
