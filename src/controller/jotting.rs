use rocket_contrib::templates::Template;

use dal::diesel_pool::{DB};
use dal::models::jotting::*;
use util::auth::User;
use util::response::{user_context, simple_context};

#[get("/jotting?<pg>")]
pub fn jottings_all_upage(db: DB, pg: i64, user: User) -> Template {
	let (result, total_article) = Jotting::pagination_query_all(db.conn(), pg);
	let mut context = user_context(&db, user);
	let total_page = (total_article - 1) / 10 + 1;
	context.insert("jottings", &result);
	context.insert("total_page", &total_page);
	context.insert("pg", &pg);
	Template::render("jotting", &context)
}

#[get("/jotting?<pg>", rank = 2)]
pub fn jottings_all_page(db: DB, pg: i64) -> Template {
	let (result, total_article) = Jotting::pagination_query_all(db.conn(), pg);
	let mut context = simple_context();
	let total_page = (total_article - 1) / 10 + 1;
	context.insert("jottings", &result);
	context.insert("total_page", &total_page);
	context.insert("pg", &pg);
	Template::render("jotting", &context)
}