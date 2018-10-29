mod db;

use rocket;
use rocket::fairing::AdHoc;

fn main() {
    rocket::ignite()
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
