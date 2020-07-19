#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket_contrib;

pub mod models;
pub mod schema;

// templates, serving static files and other rocket related stuff
use rocket::{
    config::{Config, Environment, Value},
    get, post,
    request::Form,
    routes,
};
use rocket_contrib::{databases::diesel::PgConnection, serve::StaticFiles, templates::Template};
use std::collections::HashMap;

use dotenv::dotenv;
use std::env::var;

// database stuff
use models::*;

#[database("names_db")]
pub struct NamesDbConn(PgConnection);

#[derive(Serialize)]
struct Context {
    names: Vec<Name>,
}

/// Custom config maker
/// Creates a config for Rocket with certain specifications.
fn make_config() -> Config {
    dotenv().ok();
    let port: u16 = var("PORT").unwrap().parse().unwrap();
    // create the dictionaries for the values of database information
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    database_config.insert("url", Value::from(var("DATABASE_URL").unwrap()));
    databases.insert("names_db", Value::from(database_config));

    Config::build(Environment::Production)
        .port(port)
        .extra("databases", databases) // add the databases to the config
        .finalize()
        .unwrap()
}

/// Handler for the insert route
#[post("/new", data = "<mainform>")]
fn insert(conn: NamesDbConn, mainform: Form<NewName>) -> Template {
    // insert new entry into table
    mainform.into_inner().insert_self(&conn);

    home(conn)
}

/// Handler for homepage
#[get("/")]
fn home(conn: NamesDbConn) -> Template {
    Template::render(
        "index",
        Context {
            names: Name::all(&conn),
        },
    )
}

/// Main function launching the rocket
fn main() {
    rocket::custom(make_config())
        .attach(Template::fairing())
        .attach(NamesDbConn::fairing())
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![insert, home])
        .launch();
}
