use std::ops::AddAssign;
use std::sync::Arc;
use askama::Template;
use crate::{DocumentInterface, DocumentStructure, DocumentType};
use crate::traits::PagedElement;

#[derive(Debug, Template)]
#[template(path = "module.html.jinja2")]
pub struct DocumentModule {
    namespace: Vec<String>,
    name: String,
    /// html summary
    summary: String,
    modules: Vec<Arc<DocumentModule>>,
    types: Vec<Arc<DocumentType>>,
    interfaces: Vec<Arc<DocumentInterface>>,
    structures: Vec<Arc<DocumentStructure>>,
}

impl PagedElement for DocumentModule
{
    fn new<S: ToString>(name: S) -> Arc<Self> {
        Arc::new(Self {
            namespace: vec![],
            name: name.to_string(),
            summary: "".to_string(),
            modules: vec![],
            types: vec![],
            interfaces: vec![],
            structures: vec![],
        })
    }

    fn set_summary<S: ToString>(&mut self, summary: S) {
        self.summary = summary.to_string()
    }
}

impl DocumentModule {
    pub fn has_module(&self) -> bool {
        !self.modules.is_empty()
    }
    pub fn has_interface(&self) -> bool {
        !self.interfaces.is_empty()
    }
    pub fn has_structure(&self) -> bool {
        !self.structures.is_empty()
    }
}

impl AddAssign<Arc<DocumentModule>> for DocumentModule {
    fn add_assign(&mut self, rhs: Arc<DocumentModule>) {
        self.modules.push(rhs);
    }
}

impl AddAssign<DocumentInterface> for DocumentModule {
    fn add_assign(&mut self, rhs: DocumentInterface) {
        self.interfaces.push(Arc::new(rhs));
    }
}


impl AddAssign<Arc<DocumentStructure>> for DocumentModule {
    fn add_assign(&mut self, rhs: Arc<DocumentStructure>) {
        self.structures.push(rhs)
    }
}
