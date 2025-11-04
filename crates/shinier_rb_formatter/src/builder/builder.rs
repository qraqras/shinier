use crate::document::{
    Align, Document, Group, IfBreak, Indent, Line, LineSuffix, LineSuffixBoundary,
};
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

pub fn align_number(n: i32, content: Document) -> Document {
    Document::Align(Align {
        contents: Box::new(content),
        n: Some(n),
        s: None,
    })
}

pub fn align_string(s: String, content: Document) -> Document {
    Document::Align(Align {
        contents: Box::new(content),
        n: None,
        s: Some(s),
    })
}

pub fn array(parts: &[Document]) -> Document {
    if parts.is_empty() {
        return Document::None;
    }
    Document::Array(Vec::from(parts))
}

pub fn break_parent() -> Document {
    Document::BreakParent
}

pub fn fill(parts: Document) -> Document {
    match parts {
        Document::Array(parts) => Document::Fill(crate::document::Fill { parts }),
        _ => panic!("fill function expects an Array document"),
    }
}

pub fn group(contents: Document) -> Document {
    Document::Group(Group {
        id: generate_group_id(),
        contents: Box::new(contents.clone()),
        r#break: false,
        expanded_states: None,
    })
}

pub fn conditional_group(states: &[Document]) -> Document {
    assert!(
        !states.is_empty(),
        "conditional_group requires at least one state"
    );
    Document::Group(Group {
        id: generate_group_id(),
        contents: Box::new(states.first().unwrap().clone()),
        r#break: false,
        expanded_states: Some(Vec::from(states)),
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

pub fn dedent(contents: Document) -> Document {
    align_number(-1, contents)
}

pub fn dedent_to_root(contents: Document) -> Document {
    align_number(i32::MIN, contents)
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

pub fn line_suffix(contents: Document) -> Document {
    Document::LineSuffix(LineSuffix {
        contents: Box::new(contents),
    })
}

pub fn line_suffix_boundary() -> Document {
    Document::LineSuffixBoundary(LineSuffixBoundary {
        hardline: Box::new(hardline()),
    })
}

pub fn string<T: Into<String>>(string: T) -> Document {
    Document::String(string.into())
}

pub fn space() -> Document {
    Document::String(" ".to_string())
}
