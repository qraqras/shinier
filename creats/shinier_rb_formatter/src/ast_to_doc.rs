use crate::doc::{Doc, DocType, group, indent};
use crate::visitor::Visitor;
use ruby_prism::*;

pub fn ast_to_doc(parsed: &ParseResult) {
    println!("{:?}", parsed.node());
    let mut visitor = Visitor { docs: vec![] };
    visitor.visit(&parsed.node());
    print_docs(visitor.docs);
}

pub fn print_docs(docs: Vec<Doc>) {
    for doc in docs {
        match doc.doc_type {
            DocType::Group => {
                if let Some(content) = doc.content {
                    println!("{}", content.join(""));
                }
            }
            DocType::Indent => {
                if let Some(content) = doc.content {
                    println!("    {}", content.join(""));
                }
            }
            DocType::Line => {
                todo!()
            }
            DocType::Trim => {
                todo!()
            }
        }
    }
}
