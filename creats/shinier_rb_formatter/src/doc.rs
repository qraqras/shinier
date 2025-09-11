pub type Docs = Vec<Box<dyn Doc>>;

pub trait Doc {
    fn content(&self) -> Option<String>;
    fn children(&self) -> Option<&Docs>;
}

pub struct Raw {
    pub content: String,
}
impl Doc for Raw {
    fn content(&self) -> Option<String> {
        Some(self.content.clone())
    }
    fn children(&self) -> Option<&Docs> {
        None
    }
}
pub struct Group {
    pub children: Docs,
}
impl Doc for Group {
    fn content(&self) -> Option<String> {
        None
    }
    fn children(&self) -> Option<&Docs> {
        Some(&self.children)
    }
}
pub struct Indent {
    pub content: Docs,
}
impl Doc for Indent {
    fn content(&self) -> Option<String> {
        None
    }
    fn children(&self) -> Option<&Docs> {
        Some(&self.content)
    }
}

pub fn raw(s: String) -> Box<dyn Doc> {
    Box::new(Raw { content: s })
}
pub fn group(children: Docs) -> Box<dyn Doc> {
    Box::new(Group { children })
}
