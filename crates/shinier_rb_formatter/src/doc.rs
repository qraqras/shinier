pub type Docs = Vec<Doc>;

#[derive(Debug, Clone)]
pub enum Doc {
    Text(String),
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
pub fn softline() -> Doc {
    Doc::SoftLine
}
pub fn hardline() -> Doc {
    Doc::HardLine
}
pub fn sequence(parts: Docs) -> Doc {
    Doc::Sequence(parts)
}
pub fn group(contents: Docs) -> Doc {
    Doc::Group(contents)
}
pub fn indent(contents: Docs) -> Doc {
    Doc::Indent(contents)
}
