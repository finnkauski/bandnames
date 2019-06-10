#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use(c)]
extern crate cute;

use rocket_contrib::databases::diesel;
use rocket_contrib::{serve::StaticFiles, templates::Template};

#[database("names_db")]
struct NamesDbConn(diesel::SqliteConnection);

#[derive(Serialize)]
struct Context {
    nums: Vec<isize>,
}

fn context(len: isize) -> Context {
    Context {
        nums: c![x*x*2, for x in 0..len],
    }
}

#[get("/?<len>")]
fn index(conn: NamesDbConn, len: isize) -> Template {
    Template::render("index", context(len))
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .attach(NamesDbConn::fairing())
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![index])
        .launch();
}
