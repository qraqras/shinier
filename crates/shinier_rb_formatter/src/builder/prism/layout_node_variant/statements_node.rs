use crate::Document;
use crate::builder::builder::*;

pub struct LayoutParamStatementsNode {
    pub body: Vec<Document>,
}

pub fn layout_statements_node(param: LayoutParamStatementsNode) -> Document {
    let mut documents = Vec::new();
    for (i, document) in param.body.into_iter().enumerate() {
        if i > 0 {
            documents.push(hardline());
        }
        documents.push(document);
    }
    array(&documents)
}
