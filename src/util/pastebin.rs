extern crate rand;

use rocket::http::RawStr;
use rocket::request::FromParam;

use std::fmt;
use std::borrow::Cow;
use self::rand::Rng;


const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn valid_id(id: &str) -> bool {
	id.chars().all(|c| {
		(c >= '0' && c <= '9')
			|| (c >= 'A' && c <= 'Z')
			|| (c >= 'a' && c <= 'z')
	})
}

pub struct PasteID<'a>(Cow<'a, str>);

impl<'a> PasteID<'a> {
	pub fn new(size: usize) -> PasteID<'static> {
		let mut id = String::with_capacity(size);
		let mut rng = rand::thread_rng();
		for _ in 0..size {
			id.push(BASE62[rng.gen::<usize>() % 62] as char);
		}
		PasteID(Cow::Owned(id))
	}
}

impl<'a> fmt::Display for PasteID<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl<'a> FromParam<'a> for PasteID<'a> {
	type Error = &'a RawStr;

	fn from_param(param: &'a RawStr) -> Result<PasteID<'a>, &'a RawStr> {
		match valid_id(param) {
			true => Ok(PasteID(Cow::Borrowed(param))),
			false => Err(param),
		}
	}
}