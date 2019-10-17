use rocket_contrib::templates::Template;
use rocket::response::Redirect;

use dal::diesel_pool::DB;
use dal::models::article::*;
use util::auth;
use util::log::log_to_db;
use util::log::Ip;
use util::response::{user_context, simple_context};

const VISITOR: i32 = 0;

#[get("/")]
pub fn index_upage(db: DB, ip: Ip, user: auth::User) -> Template {
	log_to_db(&db, ip, user.0);
	let (articles, have_more) = Article::query_latest_five_articles(db.conn());
	let mut context = user_context(&db, user);
	context.insert("articles", &articles);
	context.insert("have_more", &have_more);
	Template::render("index", &context)
}


#[get("/", rank = 2)]
pub fn index_page(db: DB, ip: Ip) -> Template {
	log_to_db(&db, ip, VISITOR);
	let (articles, have_more) = Article::query_latest_five_articles(db.conn());
	let mut context = simple_context();
	context.insert("articles", &articles);
	context.insert("have_more", &have_more);
	Template::render("index", &context)
}

#[get("/index")]
pub fn index(_db: DB, _ip: Ip) -> Redirect {
	Redirect::to("/")
}