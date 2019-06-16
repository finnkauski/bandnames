#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket_contrib;

pub mod models;
pub mod schema;

use rocket_contrib::databases::diesel::PgConnection;

// database stuff
#[database("names_db")]
pub struct NamesDbConn(PgConnection);
