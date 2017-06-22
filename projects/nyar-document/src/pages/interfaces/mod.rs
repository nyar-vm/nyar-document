use super::*;

#[derive(Debug)]
pub struct DocumentInterface {
    namepath: Vec<String>,
    name: String,
    /// html summary
    summary: String,
}

impl PagedElement for DocumentInterface {
    fn new<S: ToString>(name: S) -> Self {
        Self { namepath: vec![], name: name.to_string(), summary: "".to_string() }
    }

    fn get_kind(&self, plural: bool) -> &'static str {
        "Interface"
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_namespace(&self) -> &[String] {
        &self.namepath
    }

    fn set_namespace(&mut self, namepath: Vec<String>) {
        self.namepath = namepath;
    }

    fn get_summary(&self) -> &str {
        &self.summary
    }

    fn set_summary<S: ToString>(&mut self, summary: S) {
        self.summary = summary.to_string();
    }
}
// Interface <a href="../../index.html">std</a>::<wbr><a href="../index.html">io</a>::<wbr><a class="type-interface" href="#Self">filters</a>
impl DocumentInterface {}
