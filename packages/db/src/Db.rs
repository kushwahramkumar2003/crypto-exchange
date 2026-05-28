use diesel::{Connection, PgConnection};

use crate::config::Config;

pub struct Db{
    pub con:PgConnection
}

impl Db {
    pub fn new() -> Self{
        let config = Config::default();
        let connection = PgConnection::establish(&config.db_url).unwrap_or_else(|_| panic!("Error connecting to {}",config.db_url));
        Self { con: connection }
    }
}