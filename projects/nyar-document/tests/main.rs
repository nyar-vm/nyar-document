use std::fs::File;
use std::io::Write;
use nyar_document::{DocumentInterface, DocumentModule, DocumentStructure};
use askama::Template;
use nyar_document::PagedElement;

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn main() {
    let mut hello = DocumentModule::new("io"); // instantiate your struct
    let mut new1 = DocumentModule::new("new1");
    new1.set_summary("new1");
    let new2 = DocumentModule::new("new2");
    let new3 = DocumentModule::new("new3");
    hello += new1;
    hello += new2;
    hello += new3;

    // let interface = DocumentInterface::new("Show").with_summary("show interface");
    // hello += interface;
    //
    // let s1 = DocumentStructure::new("Class1").with_summary("s1");
    // let s2 = DocumentStructure::new("Class2").with_summary("s2");
    // hello += s1;
    // hello += s2;
    std::fs::create_dir_all("target/std").unwrap();
    let mut out = File::create("target/std/io.html").unwrap();
    out.write_all(hello.render().unwrap().as_bytes()).unwrap();
}