use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use rocket::Request;

use util::response::simple_context;

#[catch(404)]
pub fn not_found(req: &Request) -> Template {
	let mut context = simple_context();
	context.insert("path", req.uri().path());
	Template::render("404", &context)
}

#[catch(401)]
pub fn unauthorised(_req: &Request) -> Redirect {
	Redirect::to("/login")
}

#[catch(500)]
pub fn internal_error(req: &Request) -> Template {
	let mut context = simple_context();
	context.insert("path", req.uri().path());
	Template::render("500", &context)
}