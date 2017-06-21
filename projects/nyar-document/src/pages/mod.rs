use crate::{DocumentInterface, DocumentModule, DocumentStructure, DocumentType, PagedElement};
use askama::Template;
use dashmap::DashMap;
use std::{
    fmt::format,
    fs::File,
    io::Write,
    ops::AddAssign,
    path::{Path, PathBuf},
    sync::Arc,
};

mod enumerations;
mod flags;
pub mod interfaces;
pub mod modules;
pub mod structures;
pub mod types;
mod unions;
mod unites;
mod variants;

#[derive(Debug, Default)]
pub struct PagesManager {
    modules: DashMap<String, Arc<DocumentModule>>,
    types: Vec<Arc<DocumentType>>,
    interfaces: DashMap<String, Arc<DocumentInterface>>,
    structures: DashMap<String, Arc<DocumentStructure>>,
}

impl PagesManager {
    /// Create a new PagesManager
    pub fn register_module(&mut self, module: DocumentModule) -> Arc<DocumentModule> {
        let module = Arc::new(module);
        self.modules.insert(module.get_name().to_string(), module.clone());
        module
    }
    /// Create a new PagesManager
    pub fn register_interface(&mut self, interface: DocumentInterface) -> Arc<DocumentInterface> {
        let interface = Arc::new(interface);
        self.interfaces.insert(interface.get_name().to_string(), interface.clone());
        interface
    }
    pub fn register_structure(&mut self, structure: DocumentStructure) -> Arc<DocumentStructure> {
        let structure = Arc::new(structure);
        self.structures.insert(structure.get_name().to_string(), structure.clone());
        structure
    }

    /// Create a new PagesManager
    pub fn export<P: AsRef<Path>>(&self, root: P) -> std::io::Result<()> {
        for item in self.modules.iter() {
            let mut file = item.value().html_file(root.as_ref())?;
            let html = item.value().render().unwrap();
            file.write_all(html.as_bytes())?;
        }
        for item in self.interfaces.iter() {
            let mut file = item.value().html_file(root.as_ref())?;
            let html = item.value().render().unwrap();
            file.write_all(html.as_bytes())?;
        }
        for item in self.structures.iter() {
            let mut file = item.value().html_file(root.as_ref())?;
            let html = item.value().render().unwrap();
            file.write_all(html.as_bytes())?;
        }
        Ok(())
    }
}
