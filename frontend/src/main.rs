#[macro_use]
extern crate stdweb;

use stdweb::web::{document, HtmlElement, INode, Node};

fn main() {
    stdweb::initialize();

    match document().body() {
        Some(b) => write_document(b),
        None => console!(log, "couldn't get body")
    }

//    stdweb::event_loop();
}

fn write_document (body: HtmlElement) {
    body.append_child(&h1("Rust running on Platform.sh"));
    body.append_child(&p("This text was inserted by Javascript compiled from Rust"));
}

fn h1(body: &str) -> Node {
    Node::from_html(format!("<h1>{}</h1>", body).as_str()).unwrap()
}

fn p(body: &str) -> Node {
    Node::from_html(format!("<p>{}</p>", body).as_str()).unwrap()
}
