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

// main launcher
fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .attach(NamesDbConn::fairing())
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![update, home, delete])
        .launch();
}
