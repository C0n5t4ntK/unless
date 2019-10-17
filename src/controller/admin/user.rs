use std::collections::HashMap;

use rocket_contrib::templates::Template;

use dal::diesel_pool::DB;
use dal::models::user::*;
use util::auth::Admin;
use util::response::admin_context;

#[get("/kyriolexy")]
pub fn login_admin_page() -> Template {
	let mut context = HashMap::new();
	context.insert("welcome", "Hi mister!");
	Template::render("admin/login", &context)
}

#[get("/admin/user-profile")]
pub fn profile_page(db: DB, user: Admin) -> Template {
	let users = User::query_by_id(db.conn(), user.0);
	let mut context = admin_context(&db, user);
	if let Some(user) = users.first() {
		context.insert("user", user);
	}
	Template::render("admin/profile", &context)
}

#[get("/admin/user-list")]
pub fn user_list_page(db: DB, user: Admin) -> Template {
	let users = User::query_all(db.conn());
	let mut context = admin_context(&db, user);
	context.insert("users", &users);
	Template::render("admin/user_list", &context)
}

#[get("/admin/user-edit/<id>")]
pub fn user_edit_page(db: DB, id: i32, user: Admin) -> Template {
	let result = User::query_by_id(db.conn(), id);
	let mut context = admin_context(&db, user);
	if let Some(user) = result.first() {
		context.insert("user", user);
	}
	Template::render("admin/user_edit", &context)
}