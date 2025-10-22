#[derive(Clone, Debug)]
pub enum Doc {
    Array(Vec<Doc>),  //
    BreakParent,      //
    Group(Group),     //
    IfBreak(IfBreak), //
    Indent(Indent),   //
    Line(Line),       //
    String(String),   //
}

#[derive(Clone, Debug)]
pub struct Group {
    pub id: usize,
    pub contents: Box<Doc>,
    pub expanded_states: Option<Vec<Doc>>,
    pub r#break: bool,
}

#[derive(Clone, Debug)]
pub struct IfBreak {
    pub group_id: Option<usize>,
    pub r#break: Box<Doc>,
    pub flat: Box<Doc>,
}

#[derive(Clone, Debug)]
pub struct Indent {
    pub contents: Box<Doc>,
}

#[derive(Clone, Debug)]
pub struct Line {
    pub hard: bool,
    pub literal: bool,
    pub soft: bool,
}
