use rocket::outcome::IntoOutcome;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};

#[derive(Debug)]
pub struct User(pub i32);

impl<'a, 'r> FromRequest<'a, 'r> for User {
	type Error = ();

	fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
		request
			.cookies()
			.get_private("user_id")
			.and_then(|cookie| cookie.value().parse().ok())
			.map(|id| User(id))
			.or_forward(())
	}
}

#[derive(Debug)]
pub struct Admin(pub i32);

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
	type Error = ();

	fn from_request(request: &'a Request<'r>) -> request::Outcome<Admin, ()> {
		request
			.cookies()
			.get_private("user_id")
			.and_then(|cookie| cookie.value().parse().ok())
			.filter(|id| *id == 1)
			.map(|id| Admin(id))
			.into_outcome((Status::Unauthorized, ()))
	}
}