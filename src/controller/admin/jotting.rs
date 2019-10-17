use rocket_contrib::templates::Template;

use dal::diesel_pool::{DB};
use dal::models::jotting::*;
use util::auth::Admin;
use util::response::admin_context;

#[get("/admin/jotting-list")]
pub fn jotting_list_page(db: DB, user: Admin) -> Template {
	let jottings = Jotting::query_all(db.conn());
	let mut context = admin_context(&db, user);
	context.insert("jottings", &jottings);
	Template::render("admin/jotting_list", &context)
}

#[get("/admin/jotting-new")]
pub fn jotting_new_page(db: DB, user: Admin) -> Template {
	let context = admin_context(&db, user);
	Template::render("admin/jotting_new", &context)
}

#[get("/admin/jotting-edit/<id>")]
pub fn jotting_edit_page(db: DB, id: i32, user: Admin) -> Template {
	let result = Jotting::query_by_id(db.conn(), id);
	let mut context = admin_context(&db, user);
	if let Some(jotting) = result.first() {
		context.insert("jotting", jotting);
	}
	Template::render("admin/jotting_edit", &context)
}