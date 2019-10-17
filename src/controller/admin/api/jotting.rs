use rocket_contrib::json::Json;


use dal::diesel_pool::{DB};
use dal::models::jotting::*;
use util::auth::Admin;
use util::response::ResponseEnum;

// #[get("/api/jotting-get/all-jotting")]
// pub fn get_all_jottings(db: DB) -> Json<Vec<Jotting>> {
// 	let jottings = Jotting::query_all_published_jottings(db.conn());
// 	Json(jottings)
// }

#[post("/api/jotting-post/new", data="<new_jotting>")]
pub fn do_add_jotting(db: DB, new_jotting: Json<NewJotting>, _user: Admin) -> Json<ResponseEnum> {
	if NewJotting::insert(db.conn(), &new_jotting.0) {
		Json(ResponseEnum::SUCCESS)
	} else {
		Json(ResponseEnum::ERROR)
	}
}

#[put("/api/jotting-put/update", data="<update_jotting>")]
pub fn do_update_jotting(db: DB, update_jotting: Json<Jotting>, _user: Admin) -> Json<ResponseEnum> {
	if Jotting::update(db.conn(), &update_jotting.0) {
		Json(ResponseEnum::SUCCESS)
	} else {
		Json(ResponseEnum::ERROR)
	}
}

#[delete("/api/jotting-delete/<id>")]
pub fn do_delete_jotting(db: DB, id: i32, _user: Admin) -> Json<ResponseEnum> {
	if Jotting::delete_by_id(db.conn(), id) {
		Json(ResponseEnum::SUCCESS)
	} else {
		Json(ResponseEnum::ERROR)
	}
}

#[get("/api/jotting-get/<id>")]
pub fn get_jotting_by_id(db: DB, id: i32, _user: Admin) -> Json<Option<Jotting>> {
	let result = Jotting::query_by_id(db.conn(), id);
	Json(result.first().cloned())
}