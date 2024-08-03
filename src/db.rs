/// SQLite Implementation of Tools Write Capabilities
/// 
use datetime::;
use sqlx::sqlite::SqlitePool;
use sqlx::Error;
use scanner::ScanResult;


pub struct Db {
    db_path: String,
    pool: SqlitePool,
}


/// We implement a Db Struct and a Pool, Initialize the new database based on the path
/// Insert Results
/// @TODO Add extra CRUD options
impl Db {
    /// Creates a new Databases instance
    pub async fn new(db_path: str) -> Result<Self, Error> {
        let pool = SqlitePool.connect(&format!("sqlite://{}", db_path)).await?;
        Ok(Self { db_path, pool })
    }

    pub async fn init(&self) -> Result<(), Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS scan_results (
                timestamp STRING NOT NULL,
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ip TEXT NOT NULL,
                port INTEGER NOT NULL
                status TEXT NOT NULL DEFAULT 'CLOSED'
        );
        "#
    )
    .execute(&self.pool)
    .await?;
    Ok(())
    }

    pub async fn insert_scan_results(&self, timestamp: &str, id: u16, ip: &str, port: u16, status: ScanResult) -> Result<(), Error> {
        sqlx::query("INSERT INTO scan_results (timestamp, id, ip, port, status) VALUES (?, ?, ?, ?, ?)")
            .bind(timestamp)
            .bind(id)
            .bind(ip)
            .bind(port)
            .bind(status)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // Asynchronously get scan results
    async fn get_scan_results(db_pool: SqlitePool) -> ScanResult<impl warp::Reply, sqlx::Error> {
        match results = sqlx::query_as!(
            scan_results,
            r#"SELECT timestamp, ip, port, status FROM scan_results"#
        )
        .fetch_all(&db_pool)
        .await {
            Ok(results) => Ok(warp::reply::json(&results)),
            sqlx::Error(e) => Error("Error: {}", e),
        }
    }
}