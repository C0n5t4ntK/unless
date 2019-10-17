use rocket_contrib::templates::Template;

use dal::diesel_pool::{DB};
use dal::models::article::*;
use util::auth::Admin;
use util::response::admin_context;

#[get("/admin/article-list")]
pub fn article_list_page(db: DB, user: Admin) -> Template {
	let result = Article::query_all(db.conn());
	let mut context = admin_context(&db, user);
	let article_views: Vec<ArticleView> = result
		.into_iter()
		.map(|article| ArticleView::article_convert_to_articleview(&article))
		.collect();
	context.insert("articles", &article_views);
	Template::render("admin/article_list", &context)
}

#[get("/admin/article-new")]
pub fn article_new_page(db: DB, user: Admin) -> Template {
	let context = admin_context(&db, user);
	Template::render("admin/article_new", &context)
}

#[get("/admin/article-edit/<id>")]
pub fn article_edit_page(db: DB, id: i32, user: Admin) -> Template {
	let result = Article::query_by_id(db.conn(), id);
	let mut context = admin_context(&db, user);
	if let Some(article) = result.first() {
		context.insert("article", article);
	}
	Template::render("admin/article_edit", &context)
}