use rocket::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket_contrib::json::Json;
use ipnetwork::IpNetwork;

use std::net::IpAddr;

use dal::models::visitor_log::*;
use dal::diesel_pool::DB;
use util::time::get_now;
use util::response::ResponseEnum;

#[derive(Debug)]
pub struct Ip(pub IpAddr);

impl<'a, 'r> FromRequest<'a, 'r> for Ip {
	type Error = ();

	fn from_request(request: &'a Request<'r>) -> request::Outcome<Ip, ()> {
		if let Some(address) = request.client_ip() {
			Outcome::Success(Ip(address))
		} else {
			Outcome::Forward(())
		}
	}
}

#[get("/api/log")]
pub fn do_log(ip: Ip, db: DB) -> Json<ResponseEnum> {
	let ip_address = IpNetwork::from(ip.0);
	let new_visitor_log = NewVisitorLog {
		ip: ip_address,
		access_time: get_now(),
		user_id: 0,
	};
	if NewVisitorLog::insert(db.conn(), &new_visitor_log) {
		Json(ResponseEnum::SUCCESS)
	} else {
		Json(ResponseEnum::ERROR)
	}
}

pub fn log_to_db(db: &DB, ip: Ip, user_id: i32) {
	let ip_address = IpNetwork::from(ip.0);
	let new_visitor_log = NewVisitorLog::new(&ip_address, user_id);
	NewVisitorLog::insert(db.conn(), &new_visitor_log);
}