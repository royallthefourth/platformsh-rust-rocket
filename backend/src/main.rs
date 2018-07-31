#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
extern crate rocket;
extern crate rocket_contrib;

use rocket::response::NamedFile;
use rocket_contrib::Template;
use std::path::{Path, PathBuf};

#[derive(Serialize)]
struct TemplateContext {
    title: String,
}

#[get("/")]
fn index() -> Template {
    let context = TemplateContext{ title: String::from("Rust on Platform.sh") };
    Template::render("index", &context)
}


#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, files])
        .attach(rocket_contrib::Template::fairing())
        .launch();
}
