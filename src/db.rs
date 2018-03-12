// Taken from https://rocket.rs/guide/state/#databases.

use std::ops::Deref;

use diesel::{Connection, SqliteConnection};
use r2d2_diesel::ConnectionManager;
use r2d2::PooledConnection;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

#[cfg(not(test))]
static DATABASE_URL: &'static str = dotenv!("DATABASE_URL");
#[cfg(test)]
static DATABASE_URL: &'static str = dotenv!("TEST_DATABASE_URL");

pub struct DbConn(pub PooledConnection<ConnectionManager<SqliteConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

type Pool = ::r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<SqliteConnection>::new(&*DATABASE_URL);
    Pool::new(manager).expect("db pool")
}

pub fn connect() -> SqliteConnection {
    SqliteConnection::establish(&DATABASE_URL)
        .expect(&format!("Error connecting to {}", DATABASE_URL))
}
