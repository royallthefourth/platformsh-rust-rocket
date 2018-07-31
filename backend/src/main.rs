#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Template;

#[derive(Serialize)]
struct TemplateContext {
    title: String,
}

#[get("/")]
fn index() -> Template {
    let context = TemplateContext{ title: String::from("Rust on Platform.sh") };
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(rocket_contrib::Template::fairing())
        .launch();
}
