use askama::Template;

#[derive(Template)]
#[template(path = "module.html")]
pub struct DocumentModule {
    namepath: Vec<String>,
    /// html summary
    summary: String,
    modules: Vec<DocumentModule>,
    types: Vec<DocumentType>,
    interfaces: Vec<DocumentInterface>,
    structures: Vec<DocumentStructure>,
}

pub struct DocumentType {
    namepath: Vec<String>,
    /// html summary
    summary: String,
}

pub struct DocumentInterface {
    namepath: Vec<String>,
    /// html summary
    summary: String,
}

pub struct DocumentStructure {
    namepath: Vec<String>,
    /// html summary
    summary: String,
}