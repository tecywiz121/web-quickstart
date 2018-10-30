use rocket_contrib::databases::diesel;

#[database("web_quickstart")]
pub struct Db(diesel::SqliteConnection);
