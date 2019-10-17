use rocket_contrib::json::Json;
use chrono::NaiveDateTime;

use dal::diesel_pool::DB;
use dal::models::visitor_log::*;

#[get("/api/log/daily-pv")]
pub fn get_daily_page_view(db: DB) -> Json<Vec<(NaiveDateTime, i64)>> {
	let results = VisitorLog::count_daily_page_view(db.conn());
	Json(results)
}

#[get("/api/log/daily-uv")]
pub fn get_daily_user_view(db: DB) -> Json<Vec<(NaiveDateTime, i64)>> {
	let results = VisitorLog::count_daily_user_view(db.conn());
	Json(results)
}

#[get("/api/log/monthly-pv")]
pub fn get_monthly_page_view(db: DB) -> Json<Vec<(NaiveDateTime, i64)>> {
	let results = VisitorLog::count_monthly_page_view(db.conn());
	Json(results)
}

#[get("/api/log/monthly-uv")]
pub fn get_monthly_user_view(db: DB) -> Json<Vec<(NaiveDateTime, i64)>> {
	let results = VisitorLog::count_monthly_user_view(db.conn());
	Json(results)
}