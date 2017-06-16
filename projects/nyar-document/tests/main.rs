use std::fs::File;
use std::io::Write;
use nyar_document::{DocumentInterface, DocumentModule};
use askama::Template;

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn main() {
    let mut hello = DocumentModule::new("io"); // instantiate your struct
    let new1 = DocumentModule::new("new1").with_summary("new io");
    let new2 = DocumentModule::new("new2").with_summary("new io");
    let new3 = DocumentModule::new("new3").with_summary("new io");
    hello += new1;
    hello += new2;
    hello += new3;

    let interface = DocumentInterface::new("Show").with_summary("show interface");
    hello += interface;

    std::fs::create_dir_all("target/std").unwrap();
    let mut out = File::create("target/std/io.html").unwrap();
    out.write_all(hello.render().unwrap().as_bytes()).unwrap();
}