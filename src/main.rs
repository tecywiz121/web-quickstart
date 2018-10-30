#![allow(proc_macro_derive_resolution_fallback)] // Should be fixed in the next major Diesel version
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate typed_builder;

mod db;
mod errors;
mod models;
mod schema;
mod views;

use rocket_contrib::templates::Template;

fn main() {
    rocket::ignite()
        .mount("/", routes![views::static_page::index])
        .mount("/users", routes![views::user::detail])
        .attach(Template::fairing())
        .attach(db::Db::fairing())
        .launch();
}
