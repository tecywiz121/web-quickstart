#![allow(proc_macro_derive_resolution_fallback)] // Should be fixed in the next major Rocket version
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate error_chain;
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate typed_builder;

mod db;
mod errors;
mod models;
mod schema;
mod views;

use rocket::fairing::AdHoc;

fn main() {
    rocket::ignite()
        .mount("/users", routes![views::user::detail])
        .attach(AdHoc::on_attach(|rocket| {
            let db_url = rocket
                .config()
                .get_str("database_url")
                .unwrap_or(concat!(env!("CARGO_MANIFEST_DIR"), "/db.sqlite"));

            let pool = match db::create_pool(db_url) {
                Ok(x) => x,
                Err(_) => return Err(rocket),
            };

            Ok(rocket.manage(pool))
        }))
        .launch();
}
