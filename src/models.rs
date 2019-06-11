extern crate diesel;

use super::schema::names;
use diesel::{prelude::*, SqliteConnection};

#[derive(Debug, Queryable, Serialize)]
pub struct Name {
    pub id: Option<i32>,
    pub name: String,
    pub which: String,
}

#[derive(Insertable)]
#[table_name = "names"]
pub struct NewName {
    pub name: String,
    pub which: String,
}

impl Name {
    // make new post
    pub fn insert(conn: &SqliteConnection, name: String, which: String) -> usize {
        let new_name = NewName {
            name: name,
            which: which,
        };

        diesel::insert_into(names::table)
            .values(&new_name)
            .execute(conn)
            .expect("Error saving new post!")
    }
    // all
    pub fn all(conn: &SqliteConnection) -> Vec<Name> {
        names::table
            .order(names::id.desc())
            .load::<Name>(&*conn)
            .expect("Error! Could not get all entries.")
    }
}
