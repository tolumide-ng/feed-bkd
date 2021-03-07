use rocket::config::{Config, Environment, Value};
use std::collections::HashMap;
use std::env;

pub const DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";

/// Creates rocket config from env variables
pub fn setup_with_env() -> Config {
    let environment = Environment::active().expect("No environment found");

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT environment vairbale must be an integer");

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    database_config.insert("url", Value::from(database_url));

    databases.insert("diesel_postgres_pool", Value::from(database_config));

    Config::build(environment)
        .environment(environment)
        .port(port)
        .extra("databases", databases)
        .finalize()
        .unwrap()
}
