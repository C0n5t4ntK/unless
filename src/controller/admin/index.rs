use rocket::response::Redirect;
use rocket_contrib::templates::Template;
//use chrono::NaiveDateTime;

use dal::diesel_pool::DB;
use dal::models::visitor_log::*;
use util::auth::Admin;
use util::response::admin_context;

// #[derive(Deserialize, Serialize)]
// enum UserInfo {
// 	Username(String),
// 	AccessTime(NaiveDateTime),
// }

#[get("/admin/index")]
pub fn index_page(db: DB, user: Admin) -> Template {
	let mut context = admin_context(&db, user);
	let daily_pvs = VisitorLog::count_daily_page_view(db.conn());
	let daily_uvs = VisitorLog::count_daily_user_view(db.conn());
	let monthly_pvs = VisitorLog::count_monthly_page_view(db.conn());
	let monthly_uvs = VisitorLog::count_monthly_user_view(db.conn());
	context.insert("daily_pvs", &daily_pvs);
	context.insert("daily_uvs", &daily_uvs);
	context.insert("monthly_pvs", &monthly_pvs);
	context.insert("monthly_uvs", &monthly_uvs);
	Template::render("admin/index", &context)
}

#[get("/admin")]
pub fn index(_user: Admin) -> Redirect {
	Redirect::to("/admin/index")
}