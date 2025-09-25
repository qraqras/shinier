use crate::ast_to_doc::printer::*;
use crate::doc::*;
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
        print(&parsed.node())
    }
    fn doc_to_str(&self, doc: Doc) {
        println!("----doc_to_str----");
        const WIDTH: usize = 40;

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
                Doc::None => current_col,
                Doc::Text(s) => current_col + s.len(),
                Doc::Line => current_col + 1,
                Doc::SoftLine => current_col,
                Doc::HardLine => width + 1,
                Doc::Sequence(children) => measure_docs(children, current_col, width),
                Doc::Group(group) => measure_docs(&group.docs, current_col, width),
                Doc::Indent(children) => measure(children, current_col, width),
                Doc::IndentIfBreak(children) => measure(children, current_col, width),
                Doc::Fill(children) => measure_docs(children, current_col, width),
                Doc::IfBreak(ifbreak) => current_col + 1,
            }
        }

        fn render(doc: &Doc, st: &mut State, flat: bool) {
            match doc {
                Doc::None => {}
                Doc::Text(s) => {
                    st.write_text(s);
                }
                Doc::Line => {
                    if flat {
                        st.write_text(" ");
                    } else {
                        st.newline();
                    }
                }
                Doc::SoftLine => {
                    if !flat {
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
                Doc::Group(group) => {
                    let childlen = &group.docs;
                    let fits = measure_docs(&childlen, 0, st.width) <= st.width;
                    let next_flat = flat && fits;
                    for ch in childlen {
                        render(&ch, st, next_flat);
                    }
                }
                Doc::Indent(children) => {
                    let prev = st.indent;
                    st.indent += 2;
                    render(children, st, flat);
                    st.indent = prev;
                }
                Doc::IndentIfBreak(children) => {
                    let prev = st.indent;
                    if !flat {
                        st.indent += 2;
                    }
                    render(children, st, flat);
                    st.indent = prev;
                }
                Doc::Fill(children) => {
                    let mut iter = children.iter().peekable();
                    while let Some(ch) = iter.next() {
                        let is_last = iter.peek().is_none();
                        render(ch, st, flat);

                        // if !is_last {
                        //     if !flat {
                        //         st.newline();
                        //     }
                        // }
                    }
                }
                Doc::IfBreak(IfBreak {
                    id,
                    r#break,
                    flat: flat_doc,
                }) => {
                    if flat {
                        render(flat_doc, st, flat);
                    } else {
                        render(r#break, st, flat);
                    }
                }
            }
        }

        let mut st = State {
            out: String::new(),
            indent: 0,
            line_start: true,
            width: WIDTH,
        };
        render(&doc, &mut st, true);
        st.newline();
        println!("{}", st.out);
    }
}
