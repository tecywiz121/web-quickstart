use rocket_contrib::Template;

use std::collections::HashMap;

#[get("/")]
pub fn index() -> Template {
    Template::render("static_page/index", &HashMap::<String, String>::new())
}
