extern crate diesel;

use super::schema::names;
use diesel::{prelude::*, PgConnection};

#[derive(Debug, Queryable, Serialize)]
pub struct Name {
    pub id: i32,
    pub name: String,
    pub which: String,
}

#[derive(Insertable, Queryable, Deserialize, Debug)]
#[table_name = "names"]
pub struct NewName {
    pub name: String,
    pub which: String,
}

impl NewName {
    pub fn insert_self(self, conn: &PgConnection) -> usize {
        diesel::insert_into(names::table)
            .values(&self)
            .execute(conn)
            .expect("Error saving new bandname!")
    }
}

impl Name {
    /// Make new post
    pub fn insert(conn: &PgConnection, name: String, which: String) -> usize {
        let new_name = NewName { name, which };

        diesel::insert_into(names::table)
            .values(&new_name)
            .execute(conn)
            .expect("Error saving new post!")
    }

    /// Get all names
    pub fn all(conn: &PgConnection) -> Vec<Name> {
        names::table
            .order(names::id.desc())
            .load::<Name>(&*conn)
            .expect("Error! Could not get all entries.")
    }

    /// Delete a entry form a database
    pub fn delete(id: i32, conn: &PgConnection) -> bool {
        diesel::delete(names::table.find(id)).execute(conn).is_ok()
    }
}
