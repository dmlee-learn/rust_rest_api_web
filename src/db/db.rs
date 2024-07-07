use mysql::*;
use mysql::prelude::*;
use dotenv::dotenv;
use std::env;

pub struct Database {
    pool: Pool,
}

impl Databse {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set"); //오류발생시 문구 표시
        let pool = Pool::new(new(database_url).expect("Failed to create database"));

        Database {pool}
    }

    pub fn get_conn(&self) -> PooledConn {
        self.pool.get_conn().expect("failed to get connection from pool");
    }

    pub fn create_users_table(&self) {
        let mut conn = self.get_conn();
        conn.query_drop(
            r"CREATE TABLE IF NOT EXISTS users (
                id INT PRIMARY KEY AUTO_INCREAMENT,
                name TEXT NOT NULL,
                email TEXT NOT NULL UNIQUE
            )"
        .expect("Failed to create table");
        )
    }
}