use chrono::NaiveDateTime;
use diesel;
use diesel::pg::Pg;
use diesel::pg::PgConnection;
use diesel::expression::sql_literal::sql;
use diesel::prelude::*;

use dal::schema::article;
use dal::schema::article::dsl::article as all_articles;
use util::time::get_now;

#[derive(Serialize, Deserialize, Queryable, Debug, Clone, AsChangeset, Identifiable)]
#[table_name = "article"]
pub struct Article {
	pub id: i32,
	pub title: String,
	pub subtitle: String,
	pub raw_content: String,
	pub rendered_content: String,
	pub create_time: NaiveDateTime,
	#[serde(default = "get_now")]
	pub modify_time: NaiveDateTime,
	pub article_type: String,
	pub category: String,
	pub tag: String,
	pub page_view: i32,
	pub thumb_up: i32,
	pub published: bool,
	pub enabled_comment: bool,
	pub slug_url: String,
}

impl Article {
	pub fn query_all(conn: &PgConnection) -> Vec<Article> {
		all_articles.order(article::id.desc()).load::<Article>(conn).unwrap()
	}

	fn published() -> article::BoxedQuery<'static, Pg> {
		all_articles
			.filter(article::published.eq(true))
			.order(article::create_time.desc())
			.into_boxed()
	}

	pub fn query_all_published_articles(conn: &PgConnection) -> Vec<Article> {
		all_articles
			.filter(article::published.eq(true))
			.filter(article::article_type.eq("Article"))
			.load::<Article>(conn)
			.expect("Error loading articles!")
	}

	pub fn pagination_query_all(conn: &PgConnection, page: i64) -> (Vec<Article>, i64) {
		(Article::published()
			.filter(article::article_type.eq("Article"))
			.offset((page - 1) * 10)
			.limit(10)
			.load::<Article>(conn)
			.expect("Error loading articles!"),
		all_articles
			.filter(article::published.eq(true))
			.filter(article::article_type.eq("Article"))
			.count()
			.get_result::<i64>(conn)
			.expect("Error loading summary!"))
	}

	pub fn query_about(conn: &PgConnection) -> Vec<Article> {
		Article::published()
			.filter(article::article_type.eq("About"))
			.load::<Article>(conn)
			.expect("Error loading ABOUT page!")
	}

	pub fn query_friend(conn: &PgConnection) -> Vec<Article> {
		Article::published()
			.filter(article::article_type.eq("Friend"))
			.load::<Article>(conn)
			.expect("Error loading FRIEND page!")
	}

	pub fn query_board(conn: &PgConnection) -> Vec<Article> {
		Article::published()
			.filter(article::article_type.eq("Board"))
			.load::<Article>(conn)
			.expect("Error loading BOARD page!")
	}

	pub fn query_latest_five_articles(conn: &PgConnection) -> (Vec<Article>, bool) {
		let mut articles = Article::published()
			.filter(article::article_type.eq("Article"))
			.limit(5)
			.load::<Article>(conn)
			.expect("Error loading articles!");
		let mut have_more = false;
		if articles.len() > 4 {
			have_more = true;
			articles.pop();
		}
		(articles, have_more)
	}

	pub fn query_by_id(conn: &PgConnection, id: i32) -> Vec<Article> {
		all_articles
			.find(id)
			.load::<Article>(conn)
			.expect("Error finding article by ID!")
	}

	// pub fn query_by_category(conn: &PgConnection, category: &str) -> Vec<Article> {
	// 	Article::published()
	// 		.filter(article::article_type.eq("Article"))
	// 		.filter(article::category.eq(category))
	// 		.load::<Article>(conn)
	// 		.expect("Error finding article by category!")
	// }

	pub fn pagination_query_by_category(conn: &PgConnection, category: &str, page: i64) -> (Vec<Article>, i64) {
		(Article::published()
			.filter(article::article_type.eq("Article"))
			.filter(article::category.eq(category))
			.offset((page - 1) * 10)
			.limit(10)
			.load::<Article>(conn)
			.expect("Error loading articles by category!"),
		all_articles
			.filter(article::published.eq(true))
			.filter(article::article_type.eq("Article"))
			.filter(article::category.eq(category))
			.count()
			.get_result::<i64>(conn)
			.expect("Error loading summary by category!"))

	}

	// pub fn query_by_tag(conn: &PgConnection, tag: &str) -> Vec<Article> {
	// 	Article::published()
	// 		.filter(article::article_type.eq("Article"))
	// 		.filter(article::tag.eq(tag))
	// 		.load::<Article>(conn)
	// 		.expect("Error loading article by tag!")
	// }

	// pub fn pagination_query_by_tag(conn: &PgConnection, tag: &str, page: i64) -> (Vec<Article>, i64) {
	// 	(Article::published()
	// 		.filter(article::article_type.eq("Article"))
	// 		.filter(article::tag.eq(tag))
	// 		.offset((page - 1) * 10)
	// 		.limit(10)
	// 		.load::<Article>(conn)
	// 		.expect("Error finding article by tag!"),
	// 	all_articles
	// 		.filter(article::published.eq(true))
	// 		.filter(article::article_type.eq("Article"))
	// 		.filter(article::category.eq(tag))
	// 		.count()
	// 		.get_result::<i64>(conn)
	// 		.expect("Error loading summary by tag!"))
	// }

	pub fn query_by_slug_url(conn: &PgConnection, slug_url: &str) -> Vec<Article> {
		Article::published()
			.filter(article::article_type.eq("Article"))
			.filter(article::slug_url.eq(slug_url))
			.load::<Article>(conn)
			.expect("Error finding article by slug url!")
	}

	pub fn query_ten_hottest_articles(conn: &PgConnection) -> Vec<Article> {
		Article::published()
			.filter(article::article_type.eq("Article"))
			.order(article::page_view.desc())
			.limit(10)
			.load::<Article>(conn)
			.expect("Error loading hottest articles!")
	}

	pub fn change_page_view(conn: &PgConnection, id: i32, page_view: i32) -> bool {
		diesel::update(all_articles.find(id)).set(article::page_view.eq(page_view)).execute(conn).is_ok()
	}

	pub fn change_thumb_up(conn: &PgConnection, id: i32) -> bool {
		sql::<i32>(&*format!("UPDATE article SET thumb_up = thumb_up + 1 WHERE article.id = {};", id)).execute(conn).is_ok()
	}

	pub fn delete_by_id(conn: &PgConnection, id: i32) -> bool {
		diesel::delete(all_articles.find(id)).execute(conn).is_ok()
	}

	pub fn update(conn: &PgConnection, article: &Article) -> bool {
		diesel::update(article).set(article).execute(conn).is_ok()
	}
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "article"]
pub struct NewArticle {
	pub title: String,
	pub subtitle: String,
	pub raw_content: String,
	pub rendered_content: String,
	#[serde(default = "get_now")]
	pub create_time: NaiveDateTime,
	#[serde(default = "get_now")]
	pub modify_time: NaiveDateTime,
	pub article_type: String,
	pub category: String,
	pub tag: String,
	#[serde(default)]
	pub page_view: i32,
	#[serde(default)]
	pub thumb_up: i32,
	pub published: bool,
	pub enabled_comment: bool,
	pub slug_url: String,
}

impl NewArticle {
	pub fn insert(conn: &PgConnection, new_article: &NewArticle) -> bool {
		diesel::insert_into(article::table)
			.values(new_article)
			.execute(conn)
			.is_ok()
	}
}

#[derive(Deserialize, Serialize)]
pub struct ArticleView {
	pub id: i32,
	pub title: String,
	pub subtitle: String,
	pub create_time: NaiveDateTime,
	pub modify_time: NaiveDateTime,
	pub article_type: String,
	pub category: String,
	pub tag: String,
	pub page_view: i32,
	pub thumb_up: i32,
	pub published: bool,
	pub enabled_comment: bool,
	pub slug_url: String,
}

impl ArticleView {
	pub fn article_convert_to_articleview(article: &Article) -> ArticleView {
		ArticleView {
			id: article.id,
			title: article.title.to_string(),
			subtitle: article.subtitle.to_string(),
			create_time: article.create_time,
			modify_time: article.modify_time,
			article_type: article.article_type.to_string(),
			category: article.category.to_string(),
			tag: article.tag.to_string(),
			page_view: article.page_view,
			thumb_up: article.thumb_up,
			published: article.published,
			enabled_comment: article.enabled_comment,
			slug_url: article.slug_url.to_string(),
		}
	}
}