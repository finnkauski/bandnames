use dotenv::dotenv;
use rocket::config::{Config, Environment, Value};
use std::collections::HashMap;
use std::env::var;

/// Custom config maker
/// Creates a config for Rocket with certain specifications.
pub(crate) fn get_config() -> Config {
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
