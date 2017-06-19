use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::ops::AddAssign;
use std::path::Path;
use std::sync::Arc;
use dashmap::{DashMap, Map};
use crate::{DocumentInterface, DocumentModule, DocumentStructure, DocumentType, PagedElement};
use askama::Template;

pub mod modules;
pub mod types;
pub mod interfaces;
pub mod structures;
mod variants;
mod enumerations;
mod unions;
mod unites;
mod flags;


#[derive(Debug, Default)]
pub struct PagesManager {
    modules: DashMap<String, Arc<DocumentModule>>,
    types: Vec<Arc<DocumentType>>,
    interfaces: Vec<Arc<DocumentInterface>>,
    structures: Vec<Arc<DocumentStructure>>,
}

impl PagesManager {
    pub fn register_module(&mut self, module: DocumentModule) -> Arc<DocumentModule> {
        let module = Arc::new(module);
        self.modules.insert(module.get_name().to_string(), module.clone());
        module
    }
    pub fn export<P: AsRef<Path>>(&self, root: P) -> std::io::Result<()> {
        for item in self.modules.iter() {
            let path = item.value().html_path(root.as_ref());
            if let Some(s) = path.parent() {
                std::fs::create_dir_all(s)?
            }
            let mut file = File::create(&path)?;
            let html = item.value().render().unwrap();
            file.write_all(html.as_bytes())?;
        }
        Ok(())
    }
}