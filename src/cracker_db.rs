/// SQLite for the Password Cracker Functionality
/// 

use sqlx::SqlitePool;
use sqlx::Error;


#[derive(Debug, Serialze, sqlx::FromRow)]
pub struct PasswordHash {
    pub id: i32, // Assuming there's an ID field
    pub hash: String,
    pub status: Options(String, String),
    pub password: String,
}

pub struct Db {
    pool:SqlitePool,

}

impl Db {
    pub async fn new(db_path: &str) -> Result<Self, Error> {
        if db_path == true {
            println!("Database path: {} exists", db_path);
            match pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await? {
                Ok(db_path) => println!("Database path: {} exists", db_path),
                Err(e) => println!("Error: {}", e),
            }
        }
        Ok(Self { pool })
    }

    pub async fn init(&self) -> Result<(), Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS password_hash (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                hash TEXT NOT NULL,
                status TEXT,
                password TEXT,
            );
            "#
        )
        .execute(&self.pool)
        .await?;
    Ok(())
    }

    //CREATE
    pub async fn insert_hash(&self, id: &str, hash: &str, status: &str, password: &str,) -> Result<(), Error> {
        sqlx::query("INSERT INTO password_hash (id, hash, status, password) VALUES (?, ?, ?, ?)")
            .bind(id)
            .bind(hash)
            .bind(status)
            .bind(password)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // READ (Single Entry)
    pub async fn get_hash(&self, id: i32) -> Results<PasswordHash, Error> {
        let result = sqlx::query_as!(
            PasswordHash,
            "SELECT id, hash, status, password FROM password_hash WHERE id = ?",
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result)
    }

    // READ (All Entries)
    pub async fn get_all_hashes(&self) -> Result<Vec<PasswordHash>, Error> {
        let results = sqlx::query_as!(
            PasswordHash,
            "SELECT id, hash, status password FROM password_hash"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(results)
}
    pub async fn update_entry(&self, id: i32, status: &str, password: &str) -> Result<(), Error> {
        sqlx::query("UPDATE password_hash SET status = ?, password = ? WHERE id = ?")
            .bind(status)
            .bind(password)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete_entry(&self, id: i32) -> Result<(), Error> {
        sqlx::query("DELETE FROM password_hash WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
