use mysql::*;
use mysql::prelude::*;
use dotenv::dotenv;
use std::env;

pub struct Database {
    pool: Pool,
    conn: Pool::conn,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set"); //오류발생시 문구 표시
        let pool = Pool::new("mysql://root:password@localhost:3307/mysql").expect("Failed to create database");
        let conn = pool.get_conn();
        Database { pool, conn }
    }

    pub fn get_conn(&self) -> PooledConn {
        self.pool.get_conn().expect("failed to get connection from pool")
    }

    pub fn create_users_table(&self) {
        let mut conn = self.get_conn();
        conn.query_drop(
            r"CREATE TABLE IF NOT EXISTS users (
                id INT PRIMARY KEY AUTO_INCREAMENT,
                name TEXT NOT NULL,
                email TEXT NOT NULL UNIQUE
            )"
        ).expect("Failed to create table");
    }

    pub fn insert_user(&self, name: &str, email: &str) {
        let mut conn = self.get_conn();
        conn.exec_drop(
            r"INSERT INTO users (name, email) VALUES (:name, :email)",
            params! {
                "name" => name,
                "email" => email
            }
        ).expect("Failed to insert data");
    }

    pub fn get_users(&self) -> Vec<(i32, String, String)> {
        let mut conn = self.get_conn();
        conn.query("SELCT id, name, email FROM users")
            .expect("Failed to query data")
    }

    pub fn get_db_data(&self, qry: &str) {
        let mut conn = self.get_conn();
        conn.query(qry).expect("Failed to query");
    }
}