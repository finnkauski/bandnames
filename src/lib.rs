#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket_contrib;

// mod auth;
mod config;
mod models;
mod routes;
mod schema;

// templates, serving static files and other rocket related stuff
use rocket::routes;
use rocket_contrib::serve::StaticFiles;

pub fn get_rocket() -> rocket::Rocket {
    rocket::custom(config::get_config())
        .attach(models::NamesDbConn::fairing())
        .mount("/", StaticFiles::from("static"))
        .mount("/", routes![routes::insert, routes::all])
}
