pub type Docs = Vec<Doc>;

#[derive(Debug, Clone)]
pub enum Doc {
    Text(String),
    Line,
    SoftLine,
    HardLine,
    Sequence(Docs),
    Group(Docs),
    Indent(Docs),
}
impl Default for Doc {
    fn default() -> Self {
        Doc::Text(String::new())
    }
}

pub fn text<S: Into<String>>(s: S) -> Doc {
    Doc::Text(s.into())
}
pub fn text_u8(s: &[u8]) -> Doc {
    Doc::Text(String::from_utf8_lossy(s).to_string())
}
pub fn line() -> Doc {
    Doc::Line
}
pub fn softline() -> Doc {
    Doc::SoftLine
}
pub fn hardline() -> Doc {
    Doc::HardLine
}
pub fn sequence(docs: Docs) -> Doc {
    let mut flat = Vec::new();
    let mut deque: std::collections::VecDeque<Doc> = docs.into_iter().collect();
    while let Some(doc) = deque.pop_front() {
        match doc {
            Doc::Sequence(inner) => {
                for elem in inner.into_iter().rev() {
                    deque.push_front(elem);
                }
            }
            other => flat.push(other),
        }
    }
    Doc::Sequence(flat)
}
pub fn group(docs: Docs) -> Doc {
    Doc::Group(docs)
}
pub fn indent(contents: Docs) -> Doc {
    Doc::Indent(contents)
}
