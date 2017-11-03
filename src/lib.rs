#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
use rocket_contrib::Template;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Serialize)]
pub struct IndexCtx {
    pub number: u32,
    pub name: String,
}

#[get("/", rank = 1)]
pub fn index() -> Template {
    let context = IndexCtx { number: 4, name: "sseleman".into() };
    Template::render(stringify!(index), &context)
}

#[get("/<name>", rank = 2)]
pub fn index_name(name: String) -> Template {
    let context = IndexCtx { number: 17, name };
    Template::render(stringify!(index), &context)
}

#[get("/<name>/<number>", rank = 3)]
pub fn index_name_number(name: String, number: u32) -> Template {
    let context = IndexCtx { number, name };
    Template::render(stringify!(index), &context)
}

pub fn startup() -> rocket::error::LaunchError {
    rocket::ignite()
        .mount("/", routes![index, index_name, index_name_number])
        .attach(Template::fairing())
        .launch()
}