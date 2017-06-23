use nyar_document::{DocumentInterface, DocumentModule, DocumentStructure, PagesManager};

use nyar_document::PagedElement;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn main() {
    let mut mmg = PagesManager::default();
    let mut io = DocumentModule::new("io");
    io.set_summary("io");
    io.set_namespace(vec!["std".to_string()]);

    let mut new1 = DocumentModule::new("new1");
    new1.set_summary("new1");
    new1.set_namespace(vec!["std".to_string(), "io".to_string()]);
    let new1 = mmg.add_module(new1);
    // io += new1;

    let mut new2 = DocumentModule::new("new2");
    new2.set_summary("new2");
    new2.set_namespace(vec!["std".to_string(), "io".to_string()]);
    let new2 = mmg.add_module(new2);
    // io += new2;

    let mut i1 = DocumentInterface::new("Debug");
    i1.set_summary("Debug");
    i1.set_namespace(vec!["std".to_string(), "io".to_string()]);
    // io += mmg.add_interface(i1);

    let mut i2 = DocumentInterface::new("Display");
    i2.set_summary("Display");
    i2.set_namespace(vec!["std".to_string(), "io".to_string()]);
    // io += mmg.add_interface(i2);

    let mut s1 = DocumentStructure::new("Struct1");
    s1.set_summary("Struct1");
    s1.set_namespace(vec!["std".to_string(), "io".to_string()]);
    // io += mmg.add_structure(s1);

    mmg.add_module(io);
}
