use std::collections::HashMap;

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

        const MAX_WIDTH: usize = 20;
        const INDENT: &str = "  ";

        struct State {
            out: String,
            col: usize,
            indent: usize,
            print_width: usize,
            group_stack: Vec<(usize, bool)>,
        }

        impl State {
            fn write_indent(&mut self) {
                if self.col == 0 && self.indent > 0 {
                    self.out.push_str(&INDENT.repeat(self.indent));
                    self.col += INDENT.len() * self.indent;
                }
            }
            fn write_text(&mut self, s: &str) {
                self.write_indent();
                self.out.push_str(s);
                self.col += s.len();
            }
            fn newline(&mut self) {
                self.out.push('\n');
                self.col = 0;
            }
        }

        fn measure_docs(docs: &Docs, mut current_col: usize, width: usize) -> usize {
            for doc in docs {
                current_col = measure(doc, current_col, width);
            }
            current_col
        }

        fn measure(doc: &Doc, current_col: usize, width: usize) -> usize {
            match doc {
                Doc::None => current_col,
                Doc::Text(text) => current_col + text.len(),
                Doc::Line => current_col + 1,
                Doc::SoftLine => current_col,
                Doc::HardLine => width + 1,
                Doc::Sequence(docs) => measure_docs(docs, current_col, width),
                Doc::Group(group) => measure_docs(&group.docs, current_col, width),
                Doc::Indent(doc) => measure(doc, current_col, width),
                Doc::IndentIfBreak(doc) => measure(doc, current_col, width),
                Doc::Fill(docs) => measure_docs(docs, current_col, width),
                Doc::IfBreak(ifbreak) => measure(&ifbreak.flat, current_col, width),
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
                    let fits = measure_docs(&childlen, st.col, st.print_width) <= st.print_width;
                    let next_flat = flat && fits;

                    st.group_stack.push((group.id, next_flat));
                    for ch in childlen {
                        render(&ch, st, next_flat);
                    }
                    st.group_stack.pop();
                }
                Doc::Indent(children) => {
                    let prev = st.indent;
                    st.indent += 1;
                    render(children, st, flat);
                    st.indent = prev;
                }
                Doc::IndentIfBreak(children) => {
                    let prev = st.indent;
                    if !flat {
                        st.indent += 1;
                    }
                    render(children, st, flat);
                    st.indent = prev;
                }
                Doc::Fill(docs) => {
                    for doc in docs {
                        let next_flat = measure(doc, st.col, st.print_width) <= st.print_width;
                        render(doc, st, next_flat);
                    }
                }
                Doc::IfBreak(IfBreak {
                    id,
                    r#break,
                    flat: flat_doc,
                }) => {
                    if let Some((_, g_flat)) = st.group_stack.last() {
                        if *g_flat {
                            render(flat_doc, st, flat);
                        } else {
                            render(r#break, st, flat);
                        }
                    }
                }
            }
        }

        let mut st = State {
            out: String::new(),
            col: 0,
            indent: 0,
            print_width: MAX_WIDTH,
            group_stack: Vec::new(),
        };
        render(&doc, &mut st, true);
        st.newline();
        println!("{}", st.out);
    }
}
