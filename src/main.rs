#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use askama::Template;
use rocket_contrib::serve::StaticFiles;

mod routes;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    ayp: &'a str
}

#[get("/")]
fn index() -> IndexTemplate<'static> {
    IndexTemplate { ayp: "hey there :)" }
}

fn main() {
    rocket::ignite()
        .mount("/assets", StaticFiles::from("assets"))
        .mount("/", routes![index,
        routes::login::login_page,
        routes::register::register_page,
        routes::about::about_page,
        routes::home::home_page,
        routes::profile::profile_page,
        routes::logout::logout])
        .launch();
}
