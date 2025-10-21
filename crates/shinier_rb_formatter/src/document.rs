#[derive(Clone)]
pub enum Doc {
    Array(Vec<Doc>),  //
    BreakParent,      //
    Group(Group),     //
    IfBreak(IfBreak), //
    Indent(Indent),   //
    Line(Line),       //
    String(String),   //
}

#[derive(Clone)]
pub struct Group {
    pub id: Option<usize>,
    pub contents: Box<Doc>,
    pub expanded_states: Option<Vec<Doc>>,
    pub r#break: bool,
}

#[derive(Clone)]
pub struct IfBreak {
    pub group_id: Option<usize>,
    pub r#break: Box<Doc>,
    pub flat: Box<Doc>,
}

#[derive(Clone)]
pub struct Indent {
    pub contents: Box<Doc>,
}

#[derive(Clone)]
pub struct Line {
    pub hard: bool,
    pub literal: bool,
    pub soft: bool,
}
