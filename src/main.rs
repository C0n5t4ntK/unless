#![feature(proc_macro_hygiene, decl_macro)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate lazy_static;
extern crate tera;
extern crate chrono;
extern crate ipnetwork;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate diesel;
extern crate bcrypt;
extern crate dotenv;
extern crate fern;

#[macro_use]
extern crate log;
extern crate r2d2;
extern crate r2d2_diesel;

use std::env;
use dotenv::dotenv;
use fern::colors::{Color, ColoredLevelConfig};
use rocket_contrib::templates::Template;

use self::controller::{admin, index, article, error, jotting, user, tool};

#[cfg(test)]
mod test;

pub mod controller;
pub mod dal;
pub mod util;

mod static_file;

fn rocket() -> rocket::Rocket {
	rocket::ignite()
		.mount(
			"/",
			routes![
				// page: Template, upage: Template for user;
				// get: get status by json -> Json<Vec>, do: do RESTful api -> Json<Enum>.
				static_file::all,
				util::log::do_log,
				index::index,
				index::index_page,
				index::index_upage,
				article::articles_all_page,
				article::articles_all_upage,
				article::article_by_url_page,
				article::article_by_url_upage,
				article::board_page,
				article::board_upage,
				article::friend_page,
				article::friend_upage,
				article::about_page,
				article::about_upage,
				article::learning_summary_page,
				article::learning_summary_upage,
				article::experience_sharing_page,
				article::experience_sharing_upage,
				article::geeky_recording_page,
				article::geeky_recording_upage,
				article::miscellaneous_mood_page,
				article::miscellaneous_mood_upage,
				article::art_review_page,
				article::art_review_upage,
				jotting::jottings_all_page,
				jotting::jottings_all_upage,
				user::login_page,
				user::login_upage,
				user::signup_page,
				user::profile_upage,
				tool::tool_upage,
				tool::do_get_ip,
				tool::do_add_paste,
				tool::do_delete_paste,
				tool::do_get_paste,

				// admin panel board pages
				admin::index::index,
				admin::index::index_page,
				admin::article::article_list_page,
				admin::article::article_new_page,
				admin::article::article_edit_page,
				admin::jotting::jotting_list_page,
				admin::jotting::jotting_new_page,
				admin::jotting::jotting_edit_page,
				admin::comment::comment_list_page,
				admin::comment::comment_new_page,
				admin::comment::comment_edit_page,
				admin::user::login_admin_page,
				admin::user::profile_page,
				admin::user::user_list_page,
				admin::user::user_edit_page,

				// admin api to edit database
				//admin::api::article::get_latest_five_articles,
				admin::api::article::get_all_articles,
				admin::api::article::do_add_article,
				admin::api::article::do_update_article,
				admin::api::article::do_delete_article,
				admin::api::article::get_article_by_id,
				admin::api::article::get_hottest_ten_articles,
				admin::api::article::thumb_up,
				//admin::api::jotting::get_all_jottings,
				admin::api::jotting::do_add_jotting,
				admin::api::jotting::do_update_jotting,
				admin::api::jotting::do_delete_jotting,
				admin::api::jotting::get_jotting_by_id,
				admin::api::comment::get_all_comments,
				admin::api::comment::do_add_comment,
				admin::api::comment::do_update_comment,
				admin::api::comment::do_delete_comment,
				admin::api::comment::get_comment_by_id,
				admin::api::user::do_signup,
				admin::api::user::do_login,
				admin::api::user::do_logout,
				//admin::api::user::do_change_password,
				admin::api::user::do_update_user,
				admin::api::user::do_delete_user,
				//admin::api::user::upload_image,
				admin::api::visitor_log::get_daily_page_view,
				admin::api::visitor_log::get_daily_user_view,
				admin::api::visitor_log::get_monthly_page_view,
				admin::api::visitor_log::get_monthly_user_view,
			]
		)
		.attach(Template::fairing())
		.register(catchers![error::not_found, error::unauthorised, error::internal_error])
}

fn setup_log() {
	dotenv().ok();
	let error_log_path = env::var("ERROR_LOG_PATH").expect("ERROR_LOG_PATH must be set");
	let app_log_path = env::var("APP_LOG_PATH").expect("APP_LOG_PATH must be set");
	let colors = ColoredLevelConfig::new()
		.error(Color::Red)
		.debug(Color::Magenta)
		.info(Color::Green)
		.trace(Color::BrightBlue);
	fern::Dispatch::new()
		.chain(std::io::stdout())
		.chain(
			fern::log_file(&app_log_path)
				.expect(&format!("Cannot use this app_log_path: {}", &app_log_path))
		)
		.level(log::LevelFilter::Debug)
		.format(move |out, message, record| {
			out.finish(format_args!(
				"[{date}] [{level}] [{target}] [{message}]",
				date = chrono::Utc::now().format("%y-&m-%d %H:%M:%S"),
				level = colors.color(record.level()),
				target = record.target(),
				message = message
			))
		})
		.chain(fern::Dispatch::new().level(log::LevelFilter::Error).chain(
			fern::log_file(&error_log_path).expect(&format!(
				"Cannot use this error_log_path: {}",
				&error_log_path
			))
		))
		.apply()
		.unwrap()
}

fn main() {
	setup_log();
    info!("Your blog is starting up...");
    info!("Starting the web service controller...");
    rocket().launch();
}
