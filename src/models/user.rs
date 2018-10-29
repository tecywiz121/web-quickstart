use crate::schema::users;

use std::borrow::Cow;

#[derive(Debug, Queryable, Identifiable, PartialEq)]
pub struct User {
    id: i64,
    email: String,
}

#[derive(Debug, Deserialize, Insertable, TypedBuilder)]
#[table_name = "users"]
pub struct CreateUser<'a> {
    email: Cow<'a, str>,
}
