use std::ops::AddAssign;
use askama::Template;
use crate::{DocumentInterface, DocumentStructure, DocumentType};

#[derive(Debug, Template)]
#[template(path = "module.html.jinja2")]
pub struct DocumentModule {
    namespace: Vec<String>,
    name: String,
    /// html summary
    summary: String,
    modules: Vec<DocumentModule>,
    types: Vec<DocumentType>,
    interfaces: Vec<DocumentInterface>,
    structures: Vec<DocumentStructure>,
}

impl DocumentModule {
    pub fn new<S: ToString>(name: S) -> Self {
        Self {
            namespace: vec![],
            name: name.to_string(),
            summary: "".to_string(),
            modules: vec![],
            types: vec![],
            interfaces: vec![],
            structures: vec![],
        }
    }
    pub fn with_summary<S: ToString>(self, summary: S) -> Self {
        Self {
            summary: summary.to_string(),
            ..self
        }
    }
}

impl AddAssign<DocumentModule> for DocumentModule {
    fn add_assign(&mut self, rhs: DocumentModule) {
        self.modules.push(rhs);
    }
}

impl AddAssign<DocumentInterface> for DocumentModule {
    fn add_assign(&mut self, rhs: DocumentInterface) {
        self.interfaces.push(rhs);
    }
}
