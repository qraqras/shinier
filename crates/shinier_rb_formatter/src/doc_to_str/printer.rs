use crate::doc::*;

struct Printer {
    pub column: usize,
    pub output: String,
    group_stack: Vec<(usize, bool)>,
    indent_level: usize,
    indent_unit: String,
    max_width: usize,
}
impl Printer {
    pub fn new(indent_unit: &str, max_width: usize) -> Self {
        Self {
            column: 0,
            output: String::new(),
            group_stack: Vec::new(),
            indent_unit: indent_unit.to_string(),
            indent_level: 0,
            max_width: max_width,
        }
    }
    pub fn write_text(&mut self, text: &str) {
        self.output.push_str(text);
        self.column += text.len();
    }
    pub fn write_indent(&mut self) {
        if self.column == 0 && self.indent_level > 0 {
            self.write_text(self.indent_unit.repeat(self.indent_level).as_str());
        }
    }
    pub fn newline(&mut self) {
        self.output.push('\n');
        self.column = 0;
    }
    pub fn can_fit_in_line(&self, doc: &Doc) -> bool {
        self.column + self.measure(doc) <= self.max_width
    }
    fn measure_docs(&self, docs: &Docs) -> usize {
        let mut total = 0;
        for doc in docs {
            total += self.measure(doc);
        }
        total
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
}

pub fn print(doc: Doc) {}
