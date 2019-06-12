#![feature(proc_macro_hygiene, decl_macro)]

extern crate bandname;
extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

// templates, serving static files etc
use rocket_contrib::{serve::StaticFiles, templates::Template};
use std::collections::HashMap;

// database stuff
use bandname::{models::*, NamesDbConn};

// route handlers
#[get("/?<name>&<nametype>")]
fn update(conn: NamesDbConn, name: String, nametype: String) -> Template {
    // insert new entry into table
    Name::insert(&*conn, name, nametype);

    // get results from db
    let mut results = HashMap::new();
    results.insert("entries", Name::all(&*conn));

    Template::render("index", results)
}
#[get("/", rank = 2)]
fn home(conn: NamesDbConn) -> Template {
    // get results from db
    let mut results = HashMap::new();
    results.insert("entries", Name::all(&*conn));

    Template::render("index", results)
}

// main launcher
fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .attach(NamesDbConn::fairing())
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![update, home])
        .launch();
}
