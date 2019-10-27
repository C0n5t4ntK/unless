extern crate bcrypt;

use bcrypt::{hash, verify};
use dal::models::user;
use util::time::get_now;

#[test]
fn hash_password_test() {
	let passwd = "this_is_the_password";
	let result = hash(&passwd, 10).unwrap();
	assert_eq!(true, verify(&passwd, &result).unwrap());
	println!("{:?}", &result);
	let user = user::NewUser {
		username: String::from("C0n5t4ntK"),
		hashed_password: result.clone(),
		create_time: get_now(),
		modify_time: get_now(),
		starred: false,
		email: String::from("elapselife@outlook.com"),
		personal_site: String::from("www.elapse.life"),
		hobby: String::from("video games, basketball, programming"),
		hometown: String::from("China"),
	};
	assert_eq!(&user.hashed_password, &result);
}