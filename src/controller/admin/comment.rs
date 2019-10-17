use rocket_contrib::templates::Template;

use dal::diesel_pool::{DB};
use dal::models::comment::*;
use util::auth::Admin;
use util::response::admin_context;

#[get("/admin/comment-list")]
pub fn comment_list_page(db: DB, user: Admin) -> Template {
	let comments = Comment::query_all(db.conn());
	let mut context = admin_context(&db, user);
	context.insert("comments", &comments);
	Template::render("admin/comment_list", &context)
}

#[get("/admin/comment-new")]
pub fn comment_new_page(db: DB, user: Admin) -> Template {
	let context = admin_context(&db, user);
	Template::render("admin/comment_new", &context)
}

#[get("/admin/comment-edit/<id>")]
pub fn comment_edit_page(db: DB, id: i32, user: Admin) -> Template {
	let result = Comment::query_by_id(db.conn(), id);
	let mut context = admin_context(&db, user);
	if let Some(comment) = result.first() {
		context.insert("comment", comment);
	}
	Template::render("admin/comment_edit", &context)
}