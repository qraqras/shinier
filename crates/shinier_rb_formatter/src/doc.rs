use ruby_prism::ConstantId;
use std::sync::atomic::{AtomicUsize, Ordering};

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
    Space(SpaceDoc),
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
    pub doc: Box<Doc>,
}
#[derive(Debug, Clone)]
pub struct GroupDoc {
    pub id: usize,
    pub doc: Box<Doc>,
}
#[derive(Debug, Clone)]
pub struct HardLineDoc {}
#[derive(Debug, Clone)]
pub struct IfBreakDoc {
    pub group_id: Option<usize>,
    pub r#break: Box<Doc>,
    pub flat: Box<Doc>,
}
#[derive(Debug, Clone)]
pub struct IndentDoc {
    pub doc: Box<Doc>,
}
#[derive(Debug, Clone)]
pub struct IndentIfBreakDoc {
    pub group_id: Option<usize>,
    pub doc: Box<Doc>,
}
#[derive(Debug, Clone)]
pub struct LineDoc {}
#[derive(Debug, Clone)]
pub struct NoneDoc {}
#[derive(Debug, Clone)]
pub struct SequenceDoc {
    pub docs: Vec<Doc>,
}
#[derive(Debug, Clone)]
pub struct SpaceDoc {}
#[derive(Debug, Clone)]
pub struct SoftLineDoc {}
#[derive(Debug, Clone)]
pub struct TextDoc {
    pub text: String,
}
pub fn next_group_id() -> usize {
    NEXT_GROUP_ID.fetch_add(1, Ordering::Relaxed)
}

pub fn fill(docs: &[Doc]) -> Doc {
    Doc::Fill(FillDoc {
        doc: Box::new(sequence(docs)),
    })
}
pub fn group(docs: &[Doc]) -> Doc {
    Doc::Group(GroupDoc {
        id: next_group_id(),
        doc: Box::new(sequence(docs)),
    })
}
pub fn group_with_id(id: usize, docs: &[Doc]) -> Doc {
    Doc::Group(GroupDoc {
        id: id,
        doc: Box::new(sequence(docs)),
    })
}
pub fn hardline() -> Doc {
    Doc::HardLine(HardLineDoc {})
}
pub fn if_break(group_id: Option<usize>, r#break: Doc, flat: Doc) -> Doc {
    Doc::IfBreak(IfBreakDoc {
        group_id: group_id,
        r#break: Box::new(r#break),
        flat: Box::new(flat),
    })
}
pub fn indent(doc: &[Doc]) -> Doc {
    Doc::Indent(IndentDoc {
        doc: Box::new(sequence(doc)),
    })
}
pub fn indent_if_break(group_id: Option<usize>, doc: &[Doc]) -> Doc {
    Doc::IndentIfBreak(IndentIfBreakDoc {
        group_id: group_id,
        doc: Box::new(sequence(doc)),
    })
}
pub fn line() -> Doc {
    Doc::Line(LineDoc {})
}

pub fn none() -> Doc {
    Doc::None(NoneDoc {})
}
pub fn none_if_false(cond: bool, true_branch: Doc) -> Doc {
    if cond { true_branch } else { none() }
}
pub fn sequence(docs: &[Doc]) -> Doc {
    let mut flat_docs = Vec::new();
    let mut deque: std::collections::VecDeque<Doc> = docs.to_vec().into();
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
pub fn space() -> Doc {
    Doc::Space(SpaceDoc {})
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
pub fn text_constant(constant_id: &ConstantId) -> Doc {
    text(String::from_utf8(constant_id.as_slice().to_vec()).unwrap())
}
