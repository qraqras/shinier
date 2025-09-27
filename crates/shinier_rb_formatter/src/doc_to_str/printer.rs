use crate::doc::*;

pub struct STRPrinter {
    pub column: usize,
    pub flat: bool,
    pub output: String,
    group_stack: Vec<(usize, bool)>,
    indent_level: usize,
    indent_unit: String,
    column_max: usize,
}
impl STRPrinter {
    pub fn new(indent_unit: &str, column_max: usize) -> Self {
        Self {
            column: 0,
            flat: true,
            output: String::new(),
            group_stack: Vec::new(),
            indent_unit: indent_unit.to_string(),
            indent_level: 0,
            column_max,
        }
    }
    pub fn write_text(&mut self, text: &str) {
        self.write_indent();
        self.output.push_str(text);
        self.column += text.len();
    }
    pub fn write_indent(&mut self) {
        if self.column == 0 && self.indent_level > 0 {
            self.output
                .push_str(self.indent_unit.repeat(self.indent_level).as_str());
            self.column += self.indent_unit.len() * self.indent_level;
        }
    }
    pub fn write_newline(&mut self) {
        self.write_text("\n");
        self.column = 0;
    }
    fn increase_indent(&mut self) {
        self.indent_level += 1;
    }
    fn decrease_indent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }
    pub fn can_fit_in_line(&self, doc: &Doc) -> bool {
        let column = self.column + self.measure(doc);
        // 改行がちょうど収まってはいけない(次が必ずはみ出てしまうため)
        match doc {
            Doc::HardLine | Doc::Line | Doc::SoftLine => column < self.column_max,
            _ => column <= self.column_max,
        }
    }
    fn measure(&self, doc: &Doc) -> usize {
        match doc {
            Doc::Fill(docs) => self.measure_docs(docs),
            Doc::Group(group) => self.measure_docs(&group.docs),
            Doc::HardLine => 0,
            Doc::IfBreak(ifbreak) => self.measure(&ifbreak.flat),
            Doc::Indent(doc) => self.measure(doc),
            Doc::IndentIfBreak(doc) => self.measure(doc),
            Doc::Line => 1,
            Doc::None => 0,
            Doc::Sequence(docs) => self.measure_docs(docs),
            Doc::SoftLine => 0,
            Doc::Text(text) => text.len(),
        }
    }
    fn measure_docs(&self, docs: &Docs) -> usize {
        let mut total = 0;
        for doc in docs {
            total += self.measure(doc);
        }
        total
    }
    fn indent_print(&mut self, doc: Doc) {
        self.increase_indent();
        self.print(doc);
        self.decrease_indent();
    }
    pub fn print(&mut self, doc: Doc) {
        match doc {
            Doc::Fill(docs) => {
                for doc in docs {
                    self.flat = self.can_fit_in_line(&doc);
                    self.print(doc);
                }
            }
            Doc::Group(group) => {
                let can_fit = self.can_fit_in_line(&group.as_doc());
                self.group_stack.push((group.id, can_fit));
                self.flat = can_fit;
                for doc in group.docs {
                    self.print(doc);
                }
                self.group_stack.pop();
            }
            Doc::HardLine => {
                self.write_newline();
            }
            Doc::IfBreak(ifbreak) => {
                if self.flat {
                    self.print(*ifbreak.flat);
                } else {
                    self.print(*ifbreak.r#break);
                }
            }
            Doc::Indent(doc) => {
                self.indent_print(*doc);
            }
            Doc::IndentIfBreak(doc) => {
                if self.flat {
                    self.print(*doc);
                } else {
                    self.indent_print(*doc);
                }
            }
            Doc::Line => {
                if self.flat {
                    self.write_text(" ");
                } else {
                    self.write_newline();
                }
            }
            Doc::None => {}
            Doc::Sequence(docs) => {
                self.flat = true;
                for doc in docs {
                    self.print(doc);
                }
            }
            Doc::SoftLine => {
                if self.flat {
                    self.write_newline();
                }
            }
            Doc::Text(text) => {
                self.write_text(&text);
            }
        }
    }
}
