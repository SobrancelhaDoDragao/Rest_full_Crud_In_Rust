use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Error, NoTls, Row};

pub const DB_URL: &str = "postgres://postgres:123@localhost:5432/meudbteste";

// the input to our `create_user` handler
#[derive(Deserialize, Debug)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

// the output to our `create_user` handler
#[derive(Serialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl From<Row> for User {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
        }
    }
}

pub async fn set_database() -> Result<Client, Error> {
    let (client, connection) = tokio_postgres::connect(DB_URL, NoTls).await?;

    let sql = "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )
    ";

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.
    client.prepare(sql).await?;

    Ok(client)
}
