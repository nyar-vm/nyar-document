use std::fs::File;
use std::io::Write;
use nyar_document::{DocumentInterface, DocumentModule, DocumentStructure, PagesManager};
use askama::Template;
use nyar_document::PagedElement;

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn main() {
    let mut mmg = PagesManager::default();


    let mut new1 = DocumentModule::new("new1");
    new1.set_summary("new1");
    new1.set_namespace(vec![ "std".to_string(), "io".to_string()]);
    let new1 = mmg.register_module(new1);

    let mut new2 = DocumentModule::new("new2");
    new2.set_summary("new2");
    new2.set_namespace(vec![ "std".to_string(), "io".to_string()]);
    let new2 = mmg.register_module(new2);

    let mut io = DocumentModule::new("io");
    io.set_summary("io");
    io.set_namespace(vec!["std".to_string()]);
    io += new1;
    io += new2;
    let io = mmg.register_module(io);

    mmg.export("target").unwrap();
}