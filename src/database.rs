use std::result::Result;
use sqlx::{sqlite::SqliteQueryResult, Sqlite, SqlitePool, migrate::MigrateDatabase, Error, Pool};

pub async fn connect() -> Pool<Sqlite> {
    let db_url = String::from("sqlite://rustnotes.db");
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        println!("{} database not found... ", db_url);
        db_create(&db_url).await;

        match schema_create(&db_url).await {
            Ok(_db_url) => println!("Database created"),
            Err(err) => println!("Failed creating database. \n {}", err)
        }
    }
    SqlitePool::connect(&db_url).await.unwrap()
}

pub async fn query(qry: &str, connection_instance: Pool<Sqlite>) -> Result<SqliteQueryResult, sqlx::Error> {
    let result = sqlx::query(qry).execute(&connection_instance).await;
    let _ = &connection_instance.close().await;
    result
}

async fn db_create(db_url: &str) {
    Sqlite::create_database(db_url).await.unwrap();
    match schema_create(db_url).await {
        Ok(_) => println!("Database created successfully!"),
        Err(e) => println!("{}", e) 
    };
}

async fn schema_create(db_url:&str) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitePool::connect(db_url).await?;
    let qry = 
    "PRAGMA foreign_keys = ON ;
    CREATE TABLE IF NOT EXISTS user
        (
            user_id                 INTEGER PRIMARY KEY NOT NULL,
            password                TEXT                NOT NULL
        );    
    CREATE TABLE IF NOT EXISTS notes
        (
            note_id                     INTEGER PRIMARY KEY AUTOINCREMENT,
            title                       TEXT,
            body                        TEXT,
            updated_on                  DATETIME DEFAULT (datetime('now','localtime'))
        );";
    let result = sqlx::query(qry).execute(&pool).await;   
    pool.close().await; 
    result
}


