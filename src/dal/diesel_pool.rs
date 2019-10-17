use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Outcome::{Failure, Success};
use rocket::Request;
use dotenv::dotenv;
use diesel::pg::PgConnection;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;

use std::collections::HashMap;
use std::env;
use std::sync::Mutex;

use dal::models::article::*;

pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
	dotenv().ok();
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
	let manager = ConnectionManager::<PgConnection>::new(database_url);
	Pool::new(manager).expect("Failed to create pool!")
}

lazy_static! {
	pub static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = create_db_pool();
	pub static ref ARTICLE_CACHE: Mutex<HashMap<String, Article>> = {
		let m = HashMap::new();
		Mutex::new(m)
	};
}

pub struct DB(PooledConnection<ConnectionManager<PgConnection>>);

impl DB {
	pub fn conn(&self) -> &PgConnection {
		&*self.0
	}
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
	type Error = ();
	
	fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
		match DB_POOL.get() {
			Ok(conn) => Success(DB(conn)),
			Err(_e) => Failure((Status::InternalServerError, ())),
		}
	}
}