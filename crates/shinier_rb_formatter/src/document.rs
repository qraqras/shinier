#[derive(Clone, Debug)]
pub enum Document {
    Array(Vec<Document>), //
    BreakParent,          //
    Fill(Fill),           //
    Group(Group),         //
    IfBreak(IfBreak),     //
    Indent(Indent),       //
    Line(Line),           //
    None,                 //
    String(String),       //
}

#[derive(Clone, Debug)]
pub struct Fill {
    pub parts: Vec<Document>,
}

#[derive(Clone, Debug)]
pub struct Group {
    pub id: usize,
    pub contents: Box<Document>,
    pub expanded_states: Option<Vec<Document>>,
    pub r#break: bool,
}

#[derive(Clone, Debug)]
pub struct IfBreak {
    pub group_id: Option<usize>,
    pub r#break: Box<Document>,
    pub flat: Box<Document>,
}

#[derive(Clone, Debug)]
pub struct Indent {
    pub contents: Box<Document>,
}

#[derive(Clone, Debug)]
pub struct Line {
    pub hard: bool,
    pub literal: bool,
    pub soft: bool,
}
