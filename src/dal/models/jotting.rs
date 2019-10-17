use chrono::NaiveDateTime;
use diesel;
use diesel::pg::Pg;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use dal::schema::jotting;
use dal::schema::jotting::dsl::jotting as all_jottings;
use util::time::get_now;

#[derive(Serialize, Deserialize, Queryable, Debug, Clone, AsChangeset, Identifiable)]
#[table_name = "jotting"]
pub struct Jotting {
	pub id: i32,
	pub content: String,
	pub weather: String,
	pub mood: String,
	pub create_time: NaiveDateTime,
	pub published: bool,
}

impl Jotting {
	pub fn query_all(conn: &PgConnection) -> Vec<Jotting> {
		all_jottings.order(jotting::id.desc()).load::<Jotting>(conn).unwrap()
	}

	fn published() -> jotting::BoxedQuery<'static, Pg> {
		all_jottings
			.filter(jotting::published.eq(true))
			.order(jotting::create_time.desc())
			.into_boxed()
	}

	// pub fn query_all_published_jottings(conn: &PgConnection) -> Vec<Jotting> {
	// 	all_jottings
	// 		.filter(jotting::published.eq(true))
	// 		.load::<Jotting>(conn)
	// 		.expect("Error loading jottings!")
	// }

	pub fn pagination_query_all(conn: &PgConnection, page: i64) -> (Vec<Jotting>, i64) {
		(Jotting::published()
			.offset((page - 1) * 10)
			.limit(10)
			.load::<Jotting>(conn)
			.expect("Error loading jottings!"),
		all_jottings
			.filter(jotting::published.eq(true))
			.count()
			.get_result::<i64>(conn)
			.expect("Error loading summary!"))
	}

	pub fn query_by_id(conn: &PgConnection, id: i32) -> Vec<Jotting> {
		all_jottings
			.find(id)
			.load::<Jotting>(conn)
			.expect("Error finding jotting by ID!")
	}

	// pub fn query_by_weather(conn: &PgConnection, weather: &str) -> Vec<Jotting> {
	// 	Jotting::published()
	// 		.filter(jotting::weather.eq(weather))
	// 		.load::<Jotting>(conn)
	// 		.expect("Error finding jotting by weather!")
	// }

	// pub fn query_by_mood(conn: &PgConnection, mood: &str) -> Vec<Jotting> {
	// 	Jotting::published()
	// 		.filter(jotting::mood.eq(mood))
	// 		.load::<Jotting>(conn)
	// 		.expect("Error finding jotting by mood!")
	// }

	pub fn delete_by_id(conn: &PgConnection, id: i32) -> bool {
		diesel::delete(all_jottings.find(id)).execute(conn).is_ok()
	}

	pub fn update(conn: &PgConnection, jotting: &Jotting) -> bool {
		diesel::update(jotting).set(jotting).execute(conn).is_ok()
	}
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "jotting"]
pub struct NewJotting {
	pub content: String,
	pub weather: String,
	pub mood: String,
	#[serde(default = "get_now")]
	pub create_time: NaiveDateTime,
	pub published: bool,
}

impl NewJotting {
	pub fn insert(conn: &PgConnection, new_jotting: &NewJotting) -> bool {
		diesel::insert_into(jotting::table)
			.values(new_jotting)
			.execute(conn)
			.is_ok()
	}
}