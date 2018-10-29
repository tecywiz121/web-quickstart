use crate::db::Db;
use crate::errors::*;
use crate::models::user::User;

#[get("/<user_id>")]
pub fn detail(user_id: i32, db: Db) -> Result<Option<String>> {
    let user = match User::by_id(&db, user_id)? {
        Some(x) => x,
        None => return Ok(None),
    };

    Ok(Some(user.email().to_owned()))
}
