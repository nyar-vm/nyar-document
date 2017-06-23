#![allow(non_snake_case)]
use crate::{DocumentInterface, DocumentModule, DocumentResult, DocumentStructure, DocumentType, PagedElement};
use dioxus::prelude::*;
use semver::Version;
use serde::Deserialize;
use sir::css;
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::Write,
    ops::{Add, AddAssign},
    path::Path,
    sync::Arc,
};

pub mod enumerations;
pub mod flags;
pub mod interfaces;
pub mod modules;
pub mod packages;
pub mod structures;
pub mod types;
pub mod unions;
pub mod unites;
pub mod variants;

#[derive(Debug, Default)]
pub struct PagesManager {
    version: Option<Version>,
    modules: HashMap<String, DocumentModule>,
    types: HashMap<String, DocumentType>,
    interfaces: HashMap<String, DocumentInterface>,
    structures: HashMap<String, DocumentStructure>,
}

impl PagesManager {
    pub fn get_module(&self, name: &str) -> Option<&DocumentModule> {
        self.modules.get(name)
    }
    /// Create a new PagesManager
    pub fn add_module(&mut self, module: DocumentModule) -> String {
        let key = module.get_name_path();
        self.modules.insert(key.clone(), module);
        key
    }
    /// Create a new PagesManager
    pub fn add_interface(&mut self, interface: DocumentInterface) -> String {
        let key = interface.get_name_path();
        self.interfaces.insert(key.clone(), interface);
        key
    }
    pub fn add_structure(&mut self, structure: DocumentStructure) -> String {
        let key = structure.get_name_path();
        self.structures.insert(key.clone(), structure);
        key
    }
    pub fn add_type(&mut self, structure: DocumentType) -> String {
        let key = structure.get_name_path();
        self.types.insert(key.clone(), structure);
        key
    }

    /// Create pseudo-static html pages for all registered items
    pub fn return_html(&self, query: &str) -> DocumentResult<String> {
        todo!()
        // if let Some(s) = self.modules.get(query) {
        //     return Ok(s.render().unwrap());
        // }
        // Ok("".to_string())
    }
    /// Create static html pages for all registered items
    pub fn export_html<P: AsRef<Path>>(&self, root: P) -> std::io::Result<()> {
        // TODO: Make this parallel
        // for item in self.modules.values() {
        //     let mut file = item.html_file(root.as_ref())?;
        //     let html = item.render().unwrap();
        //     file.write_all(html.as_bytes())?;
        // }
        // for item in self.interfaces.values() {
        //     let mut file = item.html_file(root.as_ref())?;
        //     let html = item.render().unwrap();
        //     file.write_all(html.as_bytes())?;
        // }
        // for item in self.structures.values() {
        //     let mut file = item.html_file(root.as_ref())?;
        //     let html = item.render().unwrap();
        //     file.write_all(html.as_bytes())?;
        // }
        todo!();
        Ok(())
    }
}
