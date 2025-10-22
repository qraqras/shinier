use crate::document::{Document, Group, IfBreak, Indent, Line};
use std::cell::Cell;

thread_local! {
    static GROUP_ID_NEXT: Cell<usize> = Cell::new(0);
}

fn generate_group_id() -> usize {
    GROUP_ID_NEXT.with(|next| {
        let id = next.get();
        next.set(id + 1);
        id
    })
}

pub fn reset_group_id() {
    GROUP_ID_NEXT.with(|next| next.set(0));
}

pub fn array(contents: &[Document]) -> Document {
    Document::Array(contents.to_vec())
}

pub fn break_parent() -> Document {
    Document::BreakParent
}

pub fn group(contents: Document) -> Document {
    Document::Group(Group {
        id: generate_group_id(),
        contents: Box::new(contents),
        expanded_states: None,
        r#break: false,
    })
}

pub fn if_break(r#break: Document, flat: Document, group_id: Option<usize>) -> Document {
    Document::IfBreak(IfBreak {
        group_id,
        r#break: Box::new(r#break),
        flat: Box::new(flat),
    })
}

pub fn indent(contents: Document) -> Document {
    Document::Indent(Indent {
        contents: Box::new(contents),
    })
}

pub fn none() -> Document {
    Document::None
}

pub fn line() -> Document {
    Document::Line(Line {
        hard: false,
        literal: false,
        soft: false,
    })
}

pub fn hardline() -> Document {
    array(&[
        Document::Line(Line {
            hard: true,
            literal: false,
            soft: false,
        }),
        Document::BreakParent,
    ])
}

pub fn literalline() -> Document {
    array(&[
        Document::Line(Line {
            hard: true,
            literal: true,
            soft: false,
        }),
        Document::BreakParent,
    ])
}

pub fn softline() -> Document {
    Document::Line(Line {
        hard: false,
        literal: false,
        soft: true,
    })
}

pub fn string<T: Into<String>>(string: T) -> Document {
    Document::String(string.into())
}

pub fn space() -> Document {
    Document::String(" ".to_string())
}
