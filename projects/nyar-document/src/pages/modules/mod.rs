use super::*;
use std::collections::BTreeSet;

#[derive(Debug)]
pub struct DocumentModule {
    is_top: bool,
    namespace: Vec<String>,
    name: String,
    /// html summary
    summary: String,
    modules: BTreeSet<String>,
    types: BTreeSet<String>,
    interfaces: BTreeSet<String>,
    structures: BTreeSet<String>,
}

impl PagedElement for DocumentModule {
    fn new<S: ToString>(name: S) -> Self {
        Self {
            is_top: false,
            namespace: vec![],
            name: name.to_string(),
            summary: "".to_string(),
            modules: Default::default(),
            types: Default::default(),
            interfaces: Default::default(),
            structures: Default::default(),
        }
    }

    fn get_kind(&self, plural: bool) -> &'static str {
        match plural {
            true => "modules",
            false => "module",
        }
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

    fn html_file<P: AsRef<Path>>(&self, root: P) -> std::io::Result<File> {
        let mut path = self.html_directory(root).join(&self.name);
        std::fs::create_dir_all(&path)?;
        path.push("index.html");
        File::create(&path)
    }
}

impl DocumentModule {
    /// Mark this as top level module
    pub fn set_package(&mut self) {
        self.is_top = true
    }
    /// Mark this as non top module
    pub fn set_module(&mut self) {
        self.is_top = false
    }
    /// Check if has submodules
    pub fn has_module(&self) -> bool {
        !self.modules.is_empty()
    }
    /// Check is has interfaces
    pub fn has_interface(&self) -> bool {
        !self.interfaces.is_empty()
    }
    /// Check if has structures
    pub fn has_type(&self) -> bool {
        !self.types.is_empty()
    }
    /// Check if has structures
    pub fn has_structure(&self) -> bool {
        !self.structures.is_empty()
    }

    pub fn render_modules(&self, store: &PagesManager) -> String {
        todo!()
    }
}

impl AddAssign<Arc<DocumentModule>> for DocumentModule {
    fn add_assign(&mut self, rhs: Arc<DocumentModule>) {
        todo!()
        // self.modules.push(rhs);
    }
}

impl AddAssign<Arc<DocumentInterface>> for DocumentModule {
    fn add_assign(&mut self, rhs: Arc<DocumentInterface>) {
        todo!()
        // self.interfaces.push(rhs);
    }
}

impl AddAssign<Arc<DocumentStructure>> for DocumentModule {
    fn add_assign(&mut self, rhs: Arc<DocumentStructure>) {
        todo!()
        // self.structures.push(rhs)
    }
}
