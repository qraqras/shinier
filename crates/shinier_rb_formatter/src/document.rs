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
    pub n: Option<i32>,
    pub s: Option<String>,
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
    pub expanded_states: Option<Vec<Document>>,
}

#[derive(Clone, Debug)]
pub struct IfBreak {
    pub group_id: Option<usize>,
    pub r#break: Option<Box<Document>>,
    pub flat: Option<Box<Document>>,
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
