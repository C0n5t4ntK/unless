use chrono::NaiveDateTime;
use diesel;
use diesel::pg::Pg;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use dal::schema::comment;
use dal::schema::comment::dsl::comment as all_comments;
use util::time::get_now;

#[derive(Serialize, Deserialize, Queryable, Debug, Clone, AsChangeset, Identifiable)]
#[table_name = "comment"]
pub struct Comment {
	pub id: i32,
	pub user_id: i32,
	pub article_id: i32,
	pub content: String,
	pub reply_content: Option<String>,
	pub create_time: NaiveDateTime,
	pub published: bool,
}

impl Comment {
	pub fn query_all(conn: &PgConnection) -> Vec<Comment> {
		all_comments.order(comment::id.desc()).load::<Comment>(conn).unwrap()
	}

	fn published() -> comment::BoxedQuery<'static, Pg> {
		all_comments
			.filter(comment::published.eq(true))
			.order(comment::create_time.desc())
			.into_boxed()
	}

	pub fn query_by_id(conn: &PgConnection, id: i32) -> Vec<Comment> {
		all_comments
			.find(id)
			.load::<Comment>(conn)
			.expect("Error finding comment by ID!")
	}

	// pub fn query_by_user_id(conn: &PgConnection, user_id: i32) -> Vec<Comment> {
	// 	Comment::published()
	// 		.filter(comment::user_id.eq(user_id))
	// 		.load::<Comment>(conn)
	// 		.expect("Error finding comment by user_id!")
	// }

	// main func to use
	pub fn query_by_article_id(conn: &PgConnection, article_id: i32) -> Vec<Comment> {
		Comment::published()
			.filter(comment::article_id.eq(article_id))
			.load::<Comment>(conn)
			.expect("Error finding comment by article_id!")
	}

	pub fn delete_by_id(conn: &PgConnection, id: i32) -> bool {
		diesel::delete(all_comments.find(id)).execute(conn).is_ok()
	}

	pub fn update(conn: &PgConnection, comment: &Comment) -> bool {
		diesel::update(comment).set(comment).execute(conn).is_ok()
	}
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "comment"]
pub struct NewComment {
	pub user_id: i32,
	pub article_id: i32,
	pub content: String,
	#[serde(default = "get_now")]
	pub create_time: NaiveDateTime,
	pub published: bool,
}

impl NewComment {
	pub fn insert(conn: &PgConnection, new_comment: &NewComment) -> bool {
		diesel::insert_into(comment::table)
			.values(new_comment)
			.execute(conn)
			.is_ok()
	}
}