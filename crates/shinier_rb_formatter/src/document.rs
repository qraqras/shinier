#[derive(Clone, Debug)]
pub enum Document {
    Align(Align),                           //
    Array(Vec<Document>),                   //
    BreakParent,                            //
    Fill(Fill),                             //
    Group(Group),                           //
    IfBreak(IfBreak),                       //
    Indent(Indent),                         //
    Line(Line),                             //
    LineSuffix(LineSuffix),                 //
    LineSuffixBoundary(LineSuffixBoundary), //
    None,                                   //
    String(String),                         //
}

#[derive(Clone, Debug)]
pub struct Align {
    pub contents: Box<Document>,
}

#[derive(Clone, Debug)]
pub struct Fill {
    pub parts: Vec<Document>,
}

#[derive(Clone, Debug)]
pub struct Group {
    pub id: usize,
    pub contents: Box<Document>,
    pub r#break: bool,
    pub propagate_break: bool,
    pub expanded_states: Option<Vec<Document>>,
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

#[derive(Clone, Debug)]
pub struct LineSuffix {
    pub contents: Box<Document>,
}

#[derive(Clone, Debug)]
pub struct LineSuffixBoundary {
    pub hardline: Box<Document>,
}
