use rocket_contrib::templates::Template;

use std::collections::HashMap;

#[get("/")]
pub fn index() -> Template {
    Template::render("static_page/index", &HashMap::<String, String>::new())
}
