use tera::Context;
use dotenv::dotenv;

use std::env;

use dal::models::visitor_log::*;
use dal::models::user;
use dal::models::article::Article;
use dal::diesel_pool::DB;
use util::auth::{Admin, User};

#[derive(Serialize)]
pub enum ResponseEnum {
	SUCCESS,
	FAILURE,
	ERROR,
}

#[derive(Serialize)]
pub enum ContextEnum<'a, T> {
	String(String),
	Vec(Vec<T>),
	Post(Option<&'a Article>),
	User(Option<&'a user::User>),
}

pub fn admin_context(db: &DB, user: Admin) -> Context {
	let users = user::User::query_by_id(db.conn(), user.0);
	let visitor_logs = VisitorLog::query_login_user(db.conn(), user.0);
	let mut context = Context::new();
	if let Some(user) = users.first() {
		context.insert("username", &user.username);
	}
	// insert the last login time
	if let Some(log) = visitor_logs.first() {
		context.insert("access_time", &log.access_time);
	}
	context
}

pub fn user_context(db: &DB, user: User) -> Context {
	dotenv().ok();
	let email_url = env::var("EMAIL_URL").expect("EMAIL_URL must be set");
	let github_url = env::var("GITHUB_URL").expect("GITHUB_URL must be set");
	let qq_url = env::var("QQ_URL").expect("QQ_URL must be set!");
	let users = user::User::query_by_id(db.conn(), user.0);
	let visitor_logs = VisitorLog::query_login_user(db.conn(), user.0);
	let mut context = Context::new();
	if let Some(user) = users.first() {
		context.insert("user", &user);
	}
	// insert the last login time
	// get("2") because (1) is login page's record and (0) is the return page's record
	if let Some(log) = visitor_logs.get(2) {
		context.insert("access_time", &log.access_time);
	}
	context.insert("email", &email_url);
	context.insert("github", &github_url);
	context.insert("qq", &qq_url);
	context
}

pub fn simple_context() -> Context {
	dotenv().ok();
	let email_url = env::var("EMAIL_URL").expect("EMAIL_URL must be set!");
	let github_url = env::var("GITHUB_URL").expect("GITHUB_URL must be set!");
	let qq_url = env::var("QQ_URL").expect("QQ_URL must be set!");
	let mut context = Context::new();
	context.insert("email", &email_url);
	context.insert("github", &github_url);
	context.insert("qq", &qq_url);
	context
}