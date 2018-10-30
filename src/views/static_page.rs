use rocket_contrib::templates::Template;

use std::collections::HashMap;

#[get("/")]
pub fn index() -> Template {
    Template::render("static_page/index", &HashMap::<String, String>::new())
}

#[get("/demo-1")]
pub fn demo1() -> Template {
    Template::render("static_page/demo-1", &HashMap::<String, String>::new())
}
