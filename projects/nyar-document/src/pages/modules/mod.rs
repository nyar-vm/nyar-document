use super::*;

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

impl PagedElement for DocumentModule {
    fn new<S: ToString>(name: S) -> Self {
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

    fn get_kind(&self) -> &'static str {
        "Module"
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_namespace(&self) -> &[String] {
        self.namespace.as_slice()
    }

    fn set_namespace(&mut self, namepath: Vec<String>) {
        self.namespace = namepath
    }

    fn get_summary(&self) -> &str {
        self.summary.as_str()
    }

    fn set_summary<S: ToString>(&mut self, summary: S) {
        self.summary = summary.to_string()
    }

    fn href_in_module(&self) -> String {
        format!("./{}/index.html", self.name)
    }

    fn href_class(&self) -> &'static str {
        "type-module"
    }

    fn html_file<P: AsRef<Path>>(&self, root: P) -> std::io::Result<File> {
        let mut path = self.html_directory(root).join(&self.name);
        std::fs::create_dir_all(&path)?;
        path.push("index.html");
        File::create(&path)
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

impl AddAssign<Arc<DocumentInterface>> for DocumentModule {
    fn add_assign(&mut self, rhs: Arc<DocumentInterface>) {
        self.interfaces.push(rhs);
    }
}

impl AddAssign<Arc<DocumentStructure>> for DocumentModule {
    fn add_assign(&mut self, rhs: Arc<DocumentStructure>) {
        self.structures.push(rhs)
    }
}
