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
        let parsed = self.str_to_ast();
        let docs = self.ast_to_doc(&parsed);
        self.doc_to_str(docs);
    }
    fn str_to_ast(&self) -> ParseResult<'_> {
        println!("----str_to_ast----");
        parse(self.src.as_bytes())
    }
    fn ast_to_doc(&self, parsed: &ParseResult) -> Docs {
        println!("----ast_to_doc----");
        let mut visitor = Visitor::new();
        visitor.visit(&parsed.node());
        visitor.docs
    }
    fn doc_to_str(&self, docs: Docs) {
        println!("----doc_to_str----");
        const WIDTH: usize = 80;

        struct State {
            out: String,
            indent: usize,
            line_start: bool,
            width: usize,
        }

        impl State {
            fn write_indent(&mut self) {
                if self.line_start && self.indent > 0 {
                    self.out.push_str(&" ".repeat(self.indent));
                    self.line_start = false;
                }
            }
            fn write_text(&mut self, s: &str) {
                self.write_indent();
                self.out.push_str(s);
                self.line_start = false;
            }
            fn newline(&mut self) {
                self.out.push('\n');
                self.line_start = true;
            }
        }

        // Decide if a group fits on one line; naive measurement by concatenating leaf contents
        fn measure_docs(children: &Docs, mut col: usize, width: usize) -> usize {
            for ch in children {
                col = measure(ch, col, width);
                if col > width {
                    return col;
                }
            }
            col
        }

        fn measure(doc: &Doc, current_col: usize, width: usize) -> usize {
            match doc {
                Doc::Text(s) => current_col + s.len(),
                Doc::SoftLine => current_col + 1, // flatten時は空白
                Doc::HardLine => width + 1,       // 強制的にはみ出させて折り返し
                Doc::Group(children) => measure_docs(children, current_col, width),
                Doc::Indent(children) => measure_docs(children, current_col, width),
                Doc::Sequence(children) => measure_docs(children, current_col, width),
            }
        }

        fn render(doc: &Doc, st: &mut State, flat: bool) {
            match doc {
                Doc::Group(children) => {
                    let fits = measure_docs(children, 0, st.width) <= st.width;
                    let next_flat = flat && fits;
                    for ch in children {
                        render(ch, st, next_flat);
                    }
                }
                Doc::Indent(children) => {
                    let prev = st.indent;
                    st.indent += 2;
                    for ch in children {
                        render(ch, st, flat);
                    }
                    st.indent = prev;
                }
                Doc::SoftLine => {
                    if flat {
                        st.write_text(" ");
                    } else {
                        st.newline();
                    }
                }
                Doc::HardLine => {
                    st.newline();
                }
                Doc::Sequence(children) => {
                    for ch in children {
                        render(ch, st, flat);
                    }
                }
                Doc::Text(s) => {
                    st.write_text(s);
                }
            }
        }

        let mut st = State {
            out: String::new(),
            indent: 0,
            line_start: true,
            width: WIDTH,
        };
        for doc in docs.iter() {
            render(doc, &mut st, true);
            st.newline();
        }
        println!("{}", st.out);
    }
}
