use log::info;
use rocket::http::{Cookie, Cookies};
//use rocket::Data;
use rocket_contrib::json::Json;

//use std::env;
//use std::io;

use dal::diesel_pool::DB;
use dal::models::user::*;
//use util::time::get_now;
use util::auth::Admin;
use util::log::Ip;
use util::response::ResponseEnum;

#[post("/api/user-post/signup", data = "<user_info>")]
pub fn do_signup(db: DB, user_info: Json<UserInfo>) -> Json<ResponseEnum> {
	let new_user = UserInfo::convert_to_new_user(&user_info.0);
	if NewUser::insert(db.conn(), &new_user) {
		Json(ResponseEnum::SUCCESS)
	} else {
		Json(ResponseEnum::ERROR)
	}
}

#[post("/api/user-post/login", data = "<login>")]
pub fn do_login(db: DB, mut cookies: Cookies, login: Json<Login>, _ip: Ip) -> Json<ResponseEnum> {
	let users = User::query_by_email(db.conn(), &login.email);
	if let Some(user) = users.first() {
		match user.verify(&login.password) {
			Ok(valid) => {
				if valid {
					cookies.add_private(Cookie::new("user_id", user.id.to_string()));
					cookies.add_private(Cookie::new("username", user.username.to_string()));
					Json(ResponseEnum::SUCCESS)
				} else {
					Json(ResponseEnum::ERROR)
				}
			}
			Err(_) => Json(ResponseEnum::ERROR),
		}
	} else {
		Json(ResponseEnum::ERROR)
	}
}

#[get("/api/user-get/logout")]
pub fn do_logout(mut cookies: Cookies) -> Json<ResponseEnum> {
	cookies.remove_private(Cookie::named("user_id"));
	cookies.remove_private(Cookie::named("username"));
	Json(ResponseEnum::SUCCESS)
}

// #[post("/api/user-post/change-password", data = "<change_password>")]
// pub fn do_change_password(db: DB, change_password: Json<ChangePassword>, _user: Admin) -> Json<ResponseEnum> {
// 	let users = User::query_by_id(db.conn(), change_password.user_id);
// 	if let Some(user) = users.first() {
// 		match user.verify(&change_password.old_password) {
// 			Ok(valid) => {
// 				if valid {
// 					if User::change_password(
// 						db.conn(),
// 						change_password.user_id,
// 						&change_password.new_password,
// 						&get_now()
// 						) {
// 						Json(ResponseEnum::SUCCESS)
// 					} else {
// 						Json(ResponseEnum::ERROR)
// 					}
// 				} else {
// 					Json(ResponseEnum::FAILURE)
// 				}
// 			}
// 			Err(_) => Json(ResponseEnum::ERROR),
// 		}
// 	} else {
// 		Json(ResponseEnum::FAILURE)
// 	}
// }

#[put("/api/user-put/update", data = "<update_user>")]
pub fn do_update_user(db: DB, update_user: Json<User>, _user: Admin) -> Json<ResponseEnum> {
	info!("Request update user");
	if User::update(db.conn(), &update_user.0) {
		Json(ResponseEnum::SUCCESS)
	} else {
		Json(ResponseEnum::ERROR)
	}
}

#[delete("/api/user-delete/<id>")]
pub fn do_delete_user(db: DB, id: i32, _user: Admin) -> Json<ResponseEnum> {
	if User::delete_by_id(db.conn(), id) {
		Json(ResponseEnum::SUCCESS)
	} else {
		Json(ResponseEnum::ERROR)
	}
}

// #[post("/admin/user-post/upload-image", format = "image/png", data = "<data>")]
// pub fn upload_image(data: Data) -> io::Result<String> {
// 	let path = env::current_dir().unwrap();
// 	info!("The current directory is {}", path.display());
// 	data.stream_to_file("/tmp/file.png")
// 		.map(|n| format!("wrote {} bytes to /static/file", n))
// }