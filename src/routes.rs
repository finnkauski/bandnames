// database stuff
use super::models::*;
use rocket::{get, post};
use rocket_contrib::json::Json;

// Handler for the insert route
#[post("/new", data = "<data>")]
pub(crate) fn insert(conn: NamesDbConn, data: Json<NewName>) -> String {
    // insert new entry into table
    data.into_inner().insert_self(&conn);
    "Success!".into()
}

#[get("/all")]
pub(crate) fn all(conn: NamesDbConn) -> Json<Vec<Name>> {
    Json(Name::all(&conn))
}
