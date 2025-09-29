use std::sync::atomic::{AtomicUsize, Ordering};

pub type Docs = Vec<Doc>;

static NEXT_GROUP_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone)]
pub enum Doc {
    Fill(FillDoc),
    Group(GroupDoc),
    HardLine(HardLineDoc),
    IfBreak(IfBreakDoc),
    Indent(IndentDoc),
    IndentIfBreak(IndentIfBreakDoc),
    Line(LineDoc),
    None(NoneDoc),
    Sequence(SequenceDoc),
    SoftLine(SoftLineDoc),
    Text(TextDoc),
}
impl Default for Doc {
    fn default() -> Self {
        Doc::None(NoneDoc {})
    }
}

#[derive(Debug, Clone)]
pub struct FillDoc {
    pub docs: Docs,
}
#[derive(Debug, Clone)]
pub struct GroupDoc {
    pub id: usize,
    pub docs: Docs,
}
#[derive(Debug, Clone)]
pub struct HardLineDoc {}
#[derive(Debug, Clone)]
pub struct IfBreakDoc {
    pub group_id: usize,
    pub r#break: Box<Doc>,
    pub flat: Box<Doc>,
}
#[derive(Debug, Clone)]
pub struct IndentDoc {
    pub doc: Box<Doc>,
}
#[derive(Debug, Clone)]
pub struct IndentIfBreakDoc {
    pub group_id: usize,
    pub doc: Box<Doc>,
}
#[derive(Debug, Clone)]
pub struct LineDoc {}
#[derive(Debug, Clone)]
pub struct NoneDoc {}
#[derive(Debug, Clone)]
pub struct SequenceDoc {
    pub docs: Docs,
}
#[derive(Debug, Clone)]
pub struct SoftLineDoc {}
#[derive(Debug, Clone)]
pub struct TextDoc {
    pub text: String,
}
pub fn next_group_id() -> usize {
    NEXT_GROUP_ID.fetch_add(1, Ordering::Relaxed)
}

pub fn fill(docs: Docs) -> Doc {
    Doc::Fill(FillDoc { docs })
}
pub fn group(id: Option<usize>, docs: Docs) -> Doc {
    Doc::Group(GroupDoc {
        id: id.unwrap_or_else(next_group_id),
        docs: docs,
    })
}
pub fn hardline() -> Doc {
    Doc::HardLine(HardLineDoc {})
}
pub fn if_break(group_id: usize, r#break: Doc, flat: Doc) -> Doc {
    Doc::IfBreak(IfBreakDoc {
        group_id: group_id,
        r#break: Box::new(r#break),
        flat: Box::new(flat),
    })
}
pub fn indent(doc: Doc) -> Doc {
    Doc::Indent(IndentDoc { doc: Box::new(doc) })
}
pub fn indent_if_break(group_id: usize, doc: Doc) -> Doc {
    Doc::IndentIfBreak(IndentIfBreakDoc {
        group_id: group_id,
        doc: Box::new(doc),
    })
}
pub fn line() -> Doc {
    Doc::Line(LineDoc {})
}

pub fn none() -> Doc {
    Doc::None(NoneDoc {})
}
pub fn sequence(docs: Docs) -> Doc {
    let mut flat_docs = Vec::new();
    let mut deque: std::collections::VecDeque<Doc> = docs.into_iter().collect();
    while let Some(doc) = deque.pop_front() {
        match doc {
            Doc::Sequence(inner) => {
                for elem in inner.docs.into_iter().rev() {
                    deque.push_front(elem);
                }
            }
            other => flat_docs.push(other),
        }
    }
    Doc::Sequence(SequenceDoc { docs: flat_docs })
}
pub fn softline() -> Doc {
    Doc::SoftLine(SoftLineDoc {})
}
pub fn text<S: Into<String>>(text: S) -> Doc {
    Doc::Text(TextDoc { text: text.into() })
}
pub fn text_from_u8(text: &[u8]) -> Doc {
    let text = String::from_utf8(text.to_vec());
    match text {
        Ok(text) => Doc::Text(TextDoc { text }),
        Err(_) => Doc::None(NoneDoc {}), // TODO: エラー処理
    }
}
