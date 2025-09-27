use std::sync::atomic::{AtomicUsize, Ordering};

pub type Docs = Vec<Doc>;

static NEXT_GROUP_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone)]
pub enum Doc {
    Fill(Docs),
    Group(Group),
    HardLine(Line),
    IfBreak(IfBreak),
    Indent(Box<Doc>),
    IndentIfBreak(Box<Doc>),
    Line(Line),
    None,
    Sequence(Docs),
    SoftLine(Line),
    Text(String),
}
impl Default for Doc {
    fn default() -> Self {
        Doc::None
    }
}

#[derive(Debug, Clone)]
pub struct Line {}

pub trait AsDoc {
    fn as_doc(&self) -> Doc;
}
#[derive(Debug, Clone)]
pub struct Group {
    pub id: usize,
    pub docs: Docs,
}
impl AsDoc for Group {
    fn as_doc(&self) -> Doc {
        Doc::Group(self.clone())
    }
}
#[derive(Debug, Clone)]
pub struct IfBreak {
    pub id: usize,
    pub r#break: Box<Doc>,
    pub flat: Box<Doc>,
}
impl AsDoc for IfBreak {
    fn as_doc(&self) -> Doc {
        Doc::IfBreak(self.clone())
    }
}

pub fn next_group_id() -> usize {
    NEXT_GROUP_ID.fetch_add(1, Ordering::Relaxed)
}
pub fn none() -> Doc {
    Doc::None
}
pub fn text<S: Into<String>>(s: S) -> Doc {
    Doc::Text(s.into())
}
pub fn text_from_u8(s: &[u8]) -> Doc {
    Doc::Text(String::from_utf8_lossy(s).to_string())
}
pub fn line() -> Doc {
    Doc::Line(Line {})
}
pub fn softline() -> Doc {
    Doc::SoftLine(Line {})
}
pub fn hardline() -> Doc {
    Doc::HardLine(Line {})
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
pub fn group(id: Option<usize>, docs: Docs) -> Doc {
    Doc::Group(Group {
        id: id.unwrap_or_else(next_group_id),
        docs: docs,
    })
}
pub fn indent(doc: Doc) -> Doc {
    Doc::Indent(Box::new(doc))
}
pub fn indent_if_break(doc: Doc) -> Doc {
    Doc::IndentIfBreak(Box::new(doc))
}
pub fn fill(docs: Docs) -> Doc {
    Doc::Fill(docs)
}
pub fn if_break(id: usize, r#break: Doc, flat: Doc) -> Doc {
    Doc::IfBreak(IfBreak {
        id,
        r#break: Box::new(r#break),
        flat: Box::new(flat),
    })
}
