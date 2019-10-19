use std::collections::HashMap;

use rocket_contrib::templates::Template;
use rocket::response::Redirect;

use dal::diesel_pool::DB;
use dal::models::user::*;
use util::response::user_context;
use util::auth;

#[get("/login")]
pub fn login_upage(_user: auth::User) -> Redirect{
	Redirect::to("/")
}

#[get("/login", rank = 2)]
pub fn login_page() -> Template {
	let mut context = HashMap::new();
	context.insert("welcome", "Welcome back~");
	Template::render("login", &context)
}

#[get("/signup")]
pub fn signup_page() -> Template {
	let mut context = HashMap::new();
	context.insert("welcome", "Welcome stranger~");
	Template::render("admin/user_new", &context)
}

#[get("/profile/<id>")]
pub fn profile_upage(db: DB, id: i32, user: auth::User) -> Template {
	let result = User::query_by_id(db.conn(), id);
	let mut context = user_context(&db, user);
	if let Some(profile) = result.first() {
		context.insert("profile", profile);
	}
	Template::render("profile", &context)
}