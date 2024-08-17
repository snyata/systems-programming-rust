/// Injecting Routing capabilities into the system


use sqlx::{Pool, Executor, Error, Database};
use sqlx::sqlite::SqlitePool;
use sqlx::postgres::PgPool;
use sqlx::mysql::MySqlPool;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResult {
    pub timestamp: String,
    pub id: i64,
    pub ip: String,
    pub port: u16,
    pub status: String,
}

pub struct Db<D: Database> {
    pool: Pool<D>,
}

impl<D: Database> Db<D> {
    /// Creates a new Database instance with the given database URL
    pub async fn new(db_url: &str) -> Result<Self, Error> {
        let pool = Pool::<D>::connect(db_url).await?;
        Ok(Self { pool })
    }

    /// Helper function to load SQL query from a file
    fn load_query(file_path: &str) -> Result<String, std::io::Error> {
        let path = Path::new(file_path);
        fs::read_to_string(path)
    }

    /// Initializes the database by creating the necessary tables
    pub async fn init(&self, create_table_sql_file: &str) -> Result<(), Error> {
        let create_table_sql = Self::load_query(create_table_sql_file)
            .expect("Failed to load SQL query");
        
        self.pool.execute(&create_table_sql).await?;
        Ok(())
    }

    /// Inserts a new scan result into the database
    pub async fn insert_scan_result(&self, insert_sql_file: &str, ip: &str, port: u16, status: &str) -> Result<(), Error> {
        let timestamp: DateTime<Utc> = Utc::now();
        let insert_sql = Self::load_query(insert_sql_file)
            .expect("Failed to load SQL query");

        sqlx::query(&insert_sql)
            .bind(timestamp.to_rfc3339())
            .bind(ip)
            .bind(port)
            .bind(status)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Retrieves all scan results from the database
    pub async fn get_scan_results(&self, select_sql_file: &str) -> Result<Vec<ScanResult>, Error> {
        let select_sql = Self::load_query(select_sql_file)
            .expect("Failed to load SQL query");
        
        let results = sqlx::query_as::<_, ScanResult>(&select_sql)
            .fetch_all(&self.pool)
            .await?;
        Ok(results)
    }
}
