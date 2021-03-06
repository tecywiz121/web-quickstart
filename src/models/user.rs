use crate::errors::*;
use crate::schema::users;

use diesel::prelude::*;

#[derive(Debug, Serialize, Queryable, Identifiable, PartialEq)]
pub struct User {
    id: i32,
    email: String,
}

impl User {
    pub fn by_email(c: &SqliteConnection, by_email: &str) -> Result<Option<User>> {
        use self::users::dsl::*;

        let mut user = users
            .filter(email.eq(by_email))
            .limit(1)
            .load::<User>(c)
            .chain_err(|| "failed to find user by email")?;

        Ok(user.pop())
    }

    pub fn by_id(c: &SqliteConnection, by_id: i32) -> Result<Option<User>> {
        use self::users::dsl::*;

        let mut user = users
            .filter(id.eq(by_id))
            .limit(1)
            .load::<User>(c)
            .chain_err(|| "failed to find user by id")?;

        Ok(user.pop())
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }
}

#[derive(Debug, Deserialize, Insertable, TypedBuilder, FromForm)]
#[table_name = "users"]
pub struct CreateUser {
    email: String,
}

impl CreateUser {
    pub fn email(&self) -> &str {
        &self.email
    }
}
