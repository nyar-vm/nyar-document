#[derive(Debug)]
pub struct DocumentVariant {
    namepath: Vec<String>,
    name: String,
    /// html summary
    summary: String,
}

impl DocumentVariant {
    pub fn new<S: ToString>(name: S) -> Self {
        Self { namepath: vec![], name: name.to_string(), summary: "".to_string() }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_summary(&self) -> &str {
        &self.summary
    }
    pub fn with_summary<S: ToString>(self, summary: S) -> Self {
        Self { summary: summary.to_string(), ..self }
    }
}
