extern crate diesel;

use super::schema::names;
use diesel::{prelude::*, PgConnection};
use std::collections::HashMap;

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
    pub fn insert(conn: &PgConnection, name: String, which: String) -> usize {
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
    pub fn all(conn: &PgConnection) -> Vec<Name> {
        names::table
            .order(names::id.desc())
            .load::<Name>(&*conn)
            .expect("Error! Could not get all entries.")
    }
    pub fn all_c(conn: &PgConnection) -> HashMap<&str, Vec<Name>> {
        let mut results = HashMap::new();
        results.insert(
            "entries",
            names::table
                .order(names::id.desc())
                .load::<Name>(&*conn)
                .expect("Error! Could not get all entries."),
        );
        results
    }
    pub fn delete(id: i32, conn: &PgConnection) -> bool {
        diesel::delete(names::table.find(id)).execute(conn).is_ok()
    }
}
