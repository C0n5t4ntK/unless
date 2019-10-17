use rocket_contrib::json::Json;


use dal::diesel_pool::{DB};
use dal::models::comment::*;
use util::auth::{Admin, User};
use util::response::ResponseEnum;

#[get("/api/comment-get/all-comment")]
pub fn get_all_comments(db: DB, _user: Admin) -> Json<Vec<Comment>> {
	let comments = Comment::query_all(db.conn());
	Json(comments)
}

#[post("/api/comment-post/new", data="<new_comment>")]
pub fn do_add_comment(db: DB, new_comment: Json<NewComment>, _user: User) -> Json<ResponseEnum> {
	if NewComment::insert(db.conn(), &new_comment.0) {
		Json(ResponseEnum::SUCCESS)
	} else {
		Json(ResponseEnum::ERROR)
	}
}

#[put("/api/comment-put/update", data="<update_comment>")]
pub fn do_update_comment(db: DB, update_comment: Json<Comment>, _user: Admin) -> Json<ResponseEnum> {
	if Comment::update(db.conn(), &update_comment.0) {
		Json(ResponseEnum::SUCCESS)
	} else {
		Json(ResponseEnum::ERROR)
	}
}

#[delete("/api/comment-delete/<id>")]
pub fn do_delete_comment(db: DB, id: i32, _user: Admin) -> Json<ResponseEnum> {
	if Comment::delete_by_id(db.conn(), id) {
		Json(ResponseEnum::SUCCESS)
	} else {
		Json(ResponseEnum::ERROR)
	}
}

#[get("/api/comment-get/<id>")]
pub fn get_comment_by_id(db: DB, id: i32, _user: Admin) -> Json<Option<Comment>> {
	let result = Comment::query_by_id(db.conn(), id);
	Json(result.first().cloned())
}