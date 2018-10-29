use std::ops::Deref;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::SqliteConnection;

type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct Db(PooledConnection<ConnectionManager<SqliteConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for Db {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<SqlitePool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Db(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

// For the convenience of using an &Db as an &SqliteConnection.
impl Deref for Db {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn create_pool<S: Into<String>>(database_url: S) -> Result<SqlitePool, ()> {
    SqlitePool::new(ConnectionManager::new(database_url)).map_err(|_| ())
}
