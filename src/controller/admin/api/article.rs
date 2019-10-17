use rocket_contrib::json::Json;

use dal::diesel_pool::{DB, ARTICLE_CACHE};
use dal::models::article::*;
use util::auth::Admin;
use util::response::ResponseEnum;


// #[get("/api/article-get/latest-five-article")]
// pub fn get_latest_five_articles(db: DB) -> Json<Vec<ArticleView>> {
// 	let (result, _more) = Article::query_latest_five_articles(db.conn());
// 	let view_articles: Vec<ArticleView> = result
// 		.iter()
// 		.map(ArticleView::article_convert_to_articleview)
// 		.collect::<Vec<ArticleView>>();
// 	Json(view_articles)
// }

#[get("/api/article-get/all-article")]
pub fn get_all_articles(db: DB, _user: Admin) -> Json<Vec<ArticleView>> {
	let result = Article::query_all_published_articles(db.conn());
	let view_articles: Vec<ArticleView> = result
		.iter()
		.map(ArticleView::article_convert_to_articleview)
		.collect::<Vec<ArticleView>>();
	Json(view_articles)
}

#[post("/api/article-post/new", data="<new_article>")]
pub fn do_add_article(db: DB, new_article: Json<NewArticle>, _user: Admin) -> Json<ResponseEnum> {
	if NewArticle::insert(db.conn(), &new_article.0) {
		Json(ResponseEnum::SUCCESS)
	} else {
		Json(ResponseEnum::ERROR)
	}
}

#[put("/api/article-put/update", data="<update_article>")]
pub fn do_update_article(db: DB, update_article: Json<Article>, _user: Admin) -> Json<ResponseEnum> {
	if Article::update(db.conn(), &update_article.0) {
		// clear cache
		let mut hashmap = ARTICLE_CACHE.lock().unwrap();
		if hashmap.contains_key(&update_article.0.slug_url) {
			hashmap.remove(&update_article.0.slug_url);
		}
		Json(ResponseEnum::SUCCESS)
	} else {
		Json(ResponseEnum::ERROR)
	}
}

#[delete("/api/article-delete/<id>")]
pub fn do_delete_article(db: DB, id: i32, _user: Admin) -> Json<ResponseEnum> {
	if Article::delete_by_id(db.conn(), id) {
		let mut hashmap = ARTICLE_CACHE.lock().unwrap();
		hashmap.clear();
		Json(ResponseEnum::SUCCESS)
	} else {
		Json(ResponseEnum::ERROR)
	}
}

#[get("/api/article-get/<id>")]
pub fn get_article_by_id(db: DB, id: i32) -> Json<Option<Article>> {
	let result = Article::query_by_id(db.conn(), id);
	Json(result.first().cloned())
}

#[get("/api/article-get/hottest-ten-article")]
pub fn get_hottest_ten_articles(db: DB) -> Json<Vec<ArticleView>> {
	let result = Article::query_ten_hottest_articles(db.conn());
	let view_articles: Vec<ArticleView> = result
		.iter()
		.map(ArticleView::article_convert_to_articleview)
		.collect::<Vec<ArticleView>>();
	Json(view_articles)
}

#[get("/api/article-get/thumb-up/<id>")]
pub fn thumb_up(db: DB, id: i32) -> Json<ResponseEnum> {
	if Article::change_thumb_up(db.conn(), id) {
		let mut hashmap = ARTICLE_CACHE.lock().unwrap();
		hashmap.clear();
		Json(ResponseEnum::SUCCESS)
	} else {
		Json(ResponseEnum::ERROR)
	}
}