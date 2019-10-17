use rocket::response::Redirect;
use rocket_contrib::templates::Template;
//use chrono::NaiveDateTime;

use util::auth::Admin;
use util::response::admin_context;
use dal::diesel_pool::DB;

// #[derive(Deserialize, Serialize)]
// enum UserInfo {
// 	Username(String),
// 	AccessTime(NaiveDateTime),
// }

#[get("/admin/index")]
pub fn index_page(db: DB, user: Admin) -> Template {
	let context = admin_context(&db, user);
	Template::render("admin/index", &context)
}

#[get("/admin")]
pub fn index(_user: Admin) -> Redirect {
	Redirect::to("/admin/index")
}