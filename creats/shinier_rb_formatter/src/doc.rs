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

pub fn text(s: String) -> Doc {
    Doc::Text(s)
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
