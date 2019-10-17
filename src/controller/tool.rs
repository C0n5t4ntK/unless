use rocket_contrib::templates::Template;
use rocket_contrib::json::Json;
use rocket::data::Data;
use ipnetwork::IpNetwork;
use dotenv::dotenv;

use std::env;
use std::io;
use std::path::Path;
use std::fs;

use dal::diesel_pool::DB;
use util::auth::User;
use util::log::Ip;
use util::response::user_context;
use util::pastebin::PasteID;


#[get("/tool")]
pub fn tool_upage(db: DB, user: User) -> Template {
	let context = user_context(&db, user);
	Template::render("tool", &context)
}

#[get("/tool/ip")]
pub fn do_get_ip(ip: Ip, _user: User) -> Json<IpNetwork> {
	let ip_address = IpNetwork::from(ip.0);
	Json(ip_address)
}

#[derive(Deserialize, Serialize)]
pub struct NewPaste {
	pub user_id: i32,
	pub create_time: String,
	pub content: String,
}

#[post("/tool/pastebin/new", format = "json", data="<paste>")]
pub fn do_add_paste(paste: Data) -> io::Result<String> {
	dotenv().ok();
	let host = env::var("HOST").expect("HOST must be set!");
	let id = PasteID::new(5);
	let filename = format!("static/pastebin/{id}", id = id);
	let url = format!("{host}/pastebin/{id}", host = host, id = id);
	paste.stream_to_file(Path::new(&filename))?;
	Ok(url)
}

#[delete("/tool/pastebin/<id>")]
pub fn do_delete_paste(id: PasteID) -> &'static str {
	let filename = format!("static/pastebin/{id}", id = id);
	match fs::remove_file(filename) {
		Ok(()) => {"Done!\n"},
		Err(_) => {"Error\n"},
	}
}

#[get("/pastebin/<id>")]
pub fn do_get_paste<'a>(id: PasteID) -> String {
	let filename = format!("static/pastebin/{id}", id = id);
	let data = fs::read_to_string(&filename).expect("Error loading file!");
	let paste: NewPaste = serde_json::from_str(&data).expect("Error loading json!");
	paste.content
}