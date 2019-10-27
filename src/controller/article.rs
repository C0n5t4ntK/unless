use log::info;
use rocket_contrib::templates::Template;

use dal::diesel_pool::{DB, ARTICLE_CACHE};
use dal::models::article::*;
use dal::models::comment::*;
use util::auth::User;
use util::log::{log_to_db, Ip};
use util::response::{user_context, simple_context};

const VISITOR: i32 = 0;

#[get("/article?<pg>")]
pub fn articles_all_upage(db: DB, ip: Ip, pg: i64, user: User) -> Template {
	log_to_db(&db, ip, user.0);
	let (articles, total_article) = Article::pagination_query_all(db.conn(), pg);
	let mut context = user_context(&db, user);
	let total_page = (total_article - 1) / 10 + 1;
	context.insert("articles", &articles);
	context.insert("total_page", &total_page);
	context.insert("pg", &pg);
	context.insert("category", "Articles");
	Template::render("article_list", &context)
}

#[get("/article?<pg>", rank = 2)]
pub fn articles_all_page(db: DB, ip: Ip, pg: i64) -> Template {
	log_to_db(&db, ip, VISITOR);
	let (articles, total_article) = Article::pagination_query_all(db.conn(), pg);
	let mut context = simple_context();
	let total_page = (total_article - 1) / 10 + 1;
	context.insert("articles", &articles);
	context.insert("total_page", &total_page);
	context.insert("pg", &pg);
	context.insert("category", "Articles");
	Template::render("article_list", &context)
}

#[get("/article/<slug_url>")]
pub fn article_by_url_upage(db: DB, slug_url: String, ip: Ip, user: User) -> Template {
	log_to_db(&db, ip, user.0);
	let mut context = user_context(&db, user);
	let mut hashmap = ARTICLE_CACHE.lock().unwrap();
	if hashmap.contains_key(&slug_url) {
		// hit cache
		if let Some(article) = hashmap.get(&slug_url) {
			info!(
				"Hit cache, title: [{}], subtitle: [{}]",
				&article.title, &article.subtitle
			);
			let page_view = article.page_view;
			Article::change_page_view(db.conn(), article.id, page_view + 1);
			context.insert("article", article);
			let comments = Comment::query_by_article_id(db.conn(), article.id);
			context.insert("comments", &comments);
		}
	} else {
		let result = Article::query_by_slug_url(db.conn(), &slug_url);
		if let Some(article) = result.first() {
			let page_view = article.page_view;
			Article::change_page_view(db.conn(), article.id, page_view + 1);
			context.insert("article", article);
			hashmap.insert(slug_url.clone(), article.clone());
			let comments = Comment::query_by_article_id(db.conn(), article.id);
			context.insert("comments", &comments);
		}
	}
	Template::render("article", &context)
}

#[get("/article/<slug_url>", rank = 2)]
pub fn article_by_url_page(db: DB, slug_url: String, ip: Ip) -> Template {
	log_to_db(&db, ip, VISITOR);
	let mut context = simple_context();
	let mut hashmap = ARTICLE_CACHE.lock().unwrap();
	if hashmap.contains_key(&slug_url) {
		// hit cache
		if let Some(article) = hashmap.get(&slug_url) {
			info!(
				"Hit cache, title: [{}], subtitle: [{}]",
				&article.title, &article.subtitle
			);
			let page_view = article.page_view;
			Article::change_page_view(db.conn(), article.id, page_view + 1);
			context.insert("article", article);
			let comments = Comment::query_by_article_id(db.conn(), article.id);
			context.insert("comments", &comments);
		}
	} else {
		let result = Article::query_by_slug_url(db.conn(), &slug_url);
		if let Some(article) = result.first() {
			let page_view = article.page_view;
			Article::change_page_view(db.conn(), article.id, page_view + 1);
			context.insert("article", article);
			hashmap.insert(slug_url.clone(), article.clone());
			let comments = Comment::query_by_article_id(db.conn(), article.id);
			context.insert("comments", &comments);
		}
	}
	Template::render("article", &context)
}

#[get("/board")]
pub fn board_upage(db: DB, ip: Ip, user: User) -> Template {
	log_to_db(&db, ip, user.0);
	let result = Article::query_board(db.conn());
	let mut context = user_context(&db, user);
	if let Some(article) = result.first() {
		let page_view = article.page_view;
		Article::change_page_view(db.conn(), article.id, page_view + 1);
		context.insert("article", article);
	}
	let comments = Comment::query_by_article_id(db.conn(), 3);
	context.insert("comments", &comments);
	Template::render("about", &context)
}

#[get("/board", rank = 2)]
pub fn board_page(db: DB, ip: Ip) -> Template {
	log_to_db(&db, ip, VISITOR);
	let result = Article::query_board(db.conn());
	let mut context = simple_context();
	if let Some(article) = result.first() {
		let page_view = article.page_view;
		Article::change_page_view(db.conn(), article.id, page_view + 1);
		context.insert("article", article);
	}
	let comments = Comment::query_by_article_id(db.conn(), 3);
	context.insert("comments", &comments);
	Template::render("about", &context)
}

#[get("/friend")]
pub fn friend_upage(db: DB, ip: Ip, user: User) -> Template {
	log_to_db(&db, ip, user.0);
	let result = Article::query_friend(db.conn());
	let mut context = user_context(&db, user);
	if let Some(article) = result.first() {
		let page_view = article.page_view;
		Article::change_page_view(db.conn(), article.id, page_view + 1);
		context.insert("article", article);
	}
	Template::render("about", &context)
}

#[get("/friend", rank = 2)]
pub fn friend_page(db: DB, ip: Ip) -> Template {
	log_to_db(&db, ip, VISITOR);
	let result = Article::query_friend(db.conn());
	let mut context = simple_context();
	if let Some(article) = result.first() {
		let page_view = article.page_view;
		Article::change_page_view(db.conn(), article.id, page_view + 1);
		context.insert("article", article);
	}
	Template::render("about", &context)
}

#[get("/about")]
pub fn about_upage(db: DB, ip: Ip, user: User) -> Template {
	log_to_db(&db, ip, user.0);
	let result = Article::query_about(db.conn());
	let mut context = user_context(&db, user);
	if let Some(article) = result.first() {
		let page_view = article.page_view;
		Article::change_page_view(db.conn(), article.id, page_view + 1);
		context.insert("article", article);
	}
	Template::render("about", &context)
}

#[get("/about", rank = 2)]
pub fn about_page(db: DB, ip: Ip) -> Template {
	log_to_db(&db, ip, VISITOR);
	let result = Article::query_about(db.conn());
	let mut context = simple_context();
	if let Some(article) = result.first() {
		let page_view = article.page_view;
		Article::change_page_view(db.conn(), article.id, page_view + 1);
		context.insert("article", article);
	}
	Template::render("about", &context)
}

#[get("/learning-summary?<pg>")]
pub fn learning_summary_upage(db: DB, ip: Ip, pg: i64, user: User) -> Template {
	log_to_db(&db, ip, user.0);
	let category = "LearningSummary";
	let (articles, total_article) = Article::pagination_query_by_category(db.conn(), &category, pg);
	let mut context = user_context(&db, user);
	let total_page = (total_article - 1) / 10 + 1;
	context.insert("articles", &articles);
	context.insert("total_page", &total_page);
	context.insert("pg", &pg);
	context.insert("category", "Learning Summary");
	Template::render("article_list", &context)
}

#[get("/learning-summary?<pg>", rank = 2)]
pub fn learning_summary_page(db: DB, ip: Ip, pg: i64) -> Template {
	log_to_db(&db, ip, VISITOR);
	let category = "LearningSummary";
	let (articles, total_article) = Article::pagination_query_by_category(db.conn(), &category, pg);
	let mut context = simple_context();
	let total_page = (total_article - 1) / 10 + 1;
	context.insert("articles", &articles);
	context.insert("total_page", &total_page);
	context.insert("pg", &pg);
	context.insert("category", "Learning Summary");
	Template::render("article_list", &context)
}

#[get("/experience-sharing?<pg>")]
pub fn experience_sharing_upage(db: DB, ip: Ip, pg: i64, user: User) -> Template {
	log_to_db(&db, ip, user.0);
	let category = "ExperienceSharing";
	let (articles, total_article) = Article::pagination_query_by_category(db.conn(), &category, pg);
	let mut context = user_context(&db, user);
	let total_page = (total_article - 1) / 10 + 1;
	context.insert("articles", &articles);
	context.insert("total_page", &total_page);
	context.insert("pg", &pg);
	context.insert("category", "Experience Sharing");
	Template::render("article_list", &context)
}

#[get("/experience-sharing?<pg>", rank = 2)]
pub fn experience_sharing_page(db: DB, ip: Ip, pg: i64) -> Template {
	log_to_db(&db, ip, VISITOR);
	let category = "ExperienceSharing";
	let (articles, total_article) = Article::pagination_query_by_category(db.conn(), &category, pg);
	let mut context = simple_context();
	let total_page = (total_article - 1) / 10 + 1;
	context.insert("articles", &articles);
	context.insert("total_page", &total_page);
	context.insert("pg", &pg);
	context.insert("category", "Experience Sharing");
	Template::render("article_list", &context)
}

#[get("/geeky-recording?<pg>")]
pub fn geeky_recording_upage(db: DB, ip: Ip, pg: i64, user: User) -> Template {
	log_to_db(&db, ip, user.0);
	let category = "GeekyRecording";
	let (articles, total_article) = Article::pagination_query_by_category(db.conn(), &category, pg);
	let mut context = user_context(&db, user);
	let total_page = (total_article - 1) / 10 + 1;
	context.insert("articles", &articles);
	context.insert("total_page", &total_page);
	context.insert("pg", &pg);
	context.insert("category", "Geeky Recording");
	Template::render("article_list", &context)
}

#[get("/geeky-recording?<pg>", rank = 2)]
pub fn geeky_recording_page(db: DB, ip: Ip, pg: i64) -> Template {
	log_to_db(&db, ip, VISITOR);
	let category = "GeekyRecording";
	let (articles, total_article) = Article::pagination_query_by_category(db.conn(), &category, pg);
	let mut context = simple_context();
	let total_page = (total_article - 1) / 10 + 1;
	context.insert("articles", &articles);
	context.insert("total_page", &total_page);
	context.insert("pg", &pg);
	context.insert("category", "Geeky Recording");
	Template::render("article_list", &context)
}

#[get("/miscellaneous-mood?<pg>")]
pub fn miscellaneous_mood_upage(db: DB, ip: Ip, pg: i64, user: User) -> Template {
	log_to_db(&db, ip, user.0);
	let category = "MiscellaneousMood";
	let (articles,total_article) = Article::pagination_query_by_category(db.conn(), &category, pg);
	let mut context = user_context(&db, user);
	let total_page = (total_article - 1) / 10 + 1;
	context.insert("articles", &articles);
	context.insert("total_page", &total_page);
	context.insert("pg", &pg);
	context.insert("category", "Miscellaneous Mood");
	Template::render("article_list", &context)
}

#[get("/miscellaneous-mood?<pg>", rank = 2)]
pub fn miscellaneous_mood_page(db: DB, ip: Ip, pg: i64) -> Template {
	log_to_db(&db, ip, VISITOR);
	let category = "MiscellaneousMood";
	let (articles, total_article) = Article::pagination_query_by_category(db.conn(), &category, pg);
	let mut context = simple_context();
	let total_page = (total_article - 1) / 10 + 1;
	context.insert("articles", &articles);
	context.insert("total_page", &total_page);
	context.insert("pg", &pg);
	context.insert("category", "Miscellaneous Mood");
	Template::render("article_list", &context)
}

#[get("/art-review?<pg>")]
pub fn art_review_upage(db: DB, ip: Ip, pg: i64, user: User) -> Template {
	log_to_db(&db, ip, user.0);
	let category = "ArtReview";
	let (articles, total_article) = Article::pagination_query_by_category(db.conn(), &category, pg);
	let mut context = user_context(&db, user);
	let total_page = (total_article - 1) / 10 + 1;
	context.insert("articles", &articles);
	context.insert("total_page", &total_page);
	context.insert("pg", &pg);
	context.insert("category", "Art Review");
	Template::render("article_list", &context)
}

#[get("/art-review?<pg>", rank = 2)]
pub fn art_review_page(db: DB, ip: Ip, pg: i64) -> Template {
	log_to_db(&db, ip, VISITOR);
	let category = "ArtReview";
	let (articles, total_article) = Article::pagination_query_by_category(db.conn(), &category, pg);
	let mut context = simple_context();
	let total_page = (total_article - 1) / 10 + 1;
	context.insert("articles", &articles);
	context.insert("total_page", &total_page);
	context.insert("pg", &pg);
	context.insert("category", "Art Review");
	Template::render("article_list", &context)
}