#![feature(proc_macro_hygiene, decl_macro)]

extern crate bandname;
extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

// templates, serving static files etc
use dotenv::dotenv;
use rocket::config::{Config, Environment, Value};
use rocket_contrib::{serve::StaticFiles, templates::Template};
use std::collections::HashMap;
use std::env::var;

// database stuff
use bandname::{models::*, NamesDbConn};

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

// route handlers
#[get("/?<name>&<nametype>")]
fn update(conn: NamesDbConn, name: String, nametype: String) -> Template {
    // insert new entry into table
    Name::insert(&*conn, name, nametype);

    Template::render("index", Name::all_c(&*conn))
}
#[get("/?<delete>", rank = 2)]
fn delete(conn: NamesDbConn, delete: i32) -> Template {
    // delete based on id
    Name::delete(delete, &*conn);

    Template::render("index", Name::all_c(&*conn))
}
#[get("/", rank = 3)]
fn home(conn: NamesDbConn) -> Template {
    // get results from db
    let mut results = HashMap::new();
    results.insert("entries", Name::all(&*conn));

    Template::render("index", Name::all_c(&*conn))
}

/// Main function launching the rocket
fn main() {
    rocket::custom(make_config())
        .attach(Template::fairing())
        .attach(NamesDbConn::fairing())
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![update, home, delete])
        .launch();
}
