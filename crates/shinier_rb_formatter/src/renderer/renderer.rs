use crate::doc::*;

pub struct Renderer {
    pub column: usize,
    pub is_flat: bool,
    pub output: String,
    group_stack: Vec<(usize, bool)>,
    indent_level: usize,
    indent_unit: String,
    column_max: usize,
}
impl Renderer {
    pub fn new(indent_unit: &str, column_max: usize) -> Self {
        Self {
            column: 0,
            is_flat: true,
            output: String::new(),
            group_stack: Vec::new(),
            indent_unit: indent_unit.to_string(),
            indent_level: 0,
            column_max,
        }
    }
    pub fn render(&mut self, doc: Doc) {
        match doc {
            Doc::Fill(docs) => {
                for doc in docs {
                    self.is_flat = self.can_fit_in_line(&doc);
                    self.render(doc);
                }
            }
            Doc::Group(group) => {
                let can_fit = self.can_fit_in_line(&group.as_doc());
                self.group_stack.push((group.id, can_fit));
                self.is_flat = can_fit;
                for doc in group.docs {
                    self.render(doc);
                }
                self.group_stack.pop();
            }
            Doc::HardLine => {
                self.write_newline();
            }
            Doc::IfBreak(ifbreak) => {
                let groups: Vec<&(usize, bool)> = self
                    .group_stack
                    .iter()
                    .filter(|(id, _)| *id == ifbreak.id)
                    .collect();
                if let Some((_, flat)) = groups.last() {
                    if *flat {
                        self.render(*ifbreak.flat);
                    } else {
                        self.render(*ifbreak.r#break);
                    }
                } else {
                    self.render(*ifbreak.flat);
                }
            }
            Doc::Indent(doc) => {
                return self.render_with_indent(*doc);
            }
            Doc::IndentIfBreak(doc) => {
                if self.is_flat {
                    self.render(*doc);
                } else {
                    self.render_with_indent(*doc);
                }
            }
            Doc::Line => {
                if self.is_flat {
                    self.write_text(" ");
                } else {
                    self.write_newline();
                }
            }
            Doc::None => {}
            Doc::Sequence(docs) => {
                self.is_flat = true;
                for doc in docs {
                    self.render(doc);
                }
            }
            Doc::SoftLine => {
                if self.is_flat {
                    self.write_newline();
                }
            }
            Doc::Text(text) => {
                self.write_text(&text);
            }
        }
    }
    fn render_with_indent(&mut self, doc: Doc) {
        let previous_indent_level = self.indent_level;
        self.indent_level += 1;
        self.render(doc);
        self.indent_level = previous_indent_level;
    }
    fn write_text(&mut self, text: &str) {
        self.write_indent();
        self.output.push_str(text);
        self.column += text.len();
    }
    fn write_indent(&mut self) {
        if self.column == 0 && self.indent_level > 0 {
            let indent = self.indent_unit.repeat(self.indent_level);
            self.output.push_str(indent.as_str());
            self.column += indent.len();
        }
    }
    fn write_newline(&mut self) {
        self.output.push_str("\n");
        self.column = 0;
    }
    fn can_fit_in_line(&self, doc: &Doc) -> bool {
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
}
